use crate::draw::DrawList;
use crate::layout::{
    ConstraintSet, ConstraintSolver, Constraints, LayoutNode, LayoutResult,
    Target as ConstraintTarget,
};
use crate::state::{get_or_set_state, update_state};
use crate::types::{Rect, Size};
use crate::widget::Widget;

#[derive(Debug, Clone, Default)]
struct LayoutCache {
    solved_rects: Vec<Rect>,
    parent_size: Size,
}

pub struct ConstraintLayout<M> {
    pub id: Option<String>,
    pub children: Vec<Box<dyn Widget<M>>>,
    pub constraints: Vec<ConstraintSet>,
}

impl<M> ConstraintLayout<M> {
    pub fn new() -> Self {
        Self {
            id: None,
            children: Vec::new(),
            constraints: Vec::new(),
        }
    }

    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn with_child<F>(mut self, child: Box<dyn Widget<M>>, f: F) -> Self
    where
        F: FnOnce(ConstraintSet) -> ConstraintSet,
    {
        self.children.push(child);
        self.constraints.push(f(ConstraintSet::new()));
        self
    }
}

impl<M: 'static> Widget<M> for ConstraintLayout<M> {
    fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    fn layout(
        &self,
        constraints: Constraints,
        _children: &[LayoutNode],
        text_system: &mut crate::text::TextSystem,
        theme: &crate::theme::Theme,
    ) -> LayoutResult {
        if let Some(id) = self.id() {
            let cache = get_or_set_state::<LayoutCache, _>(id, || LayoutCache::default());
            if cache.parent_size == constraints.max_size() && !cache.solved_rects.is_empty() {
                let mut max_x = 0.0f32;
                let mut max_y = 0.0f32;
                for r in &cache.solved_rects {
                    max_x = max_x.max(r.x + r.width);
                    max_y = max_y.max(r.y + r.height);
                }
                return LayoutResult {
                    size: constraints.clamp_size(Size {
                        width: max_x,
                        height: max_y,
                    }),
                };
            }
        }

        let mut child_sizes = Vec::with_capacity(self.children.len());
        for child in &self.children {
            let res = child.layout(
                Constraints::loose(constraints.max_width, constraints.max_height),
                &[],
                text_system,
                theme,
            );
            child_sizes.push(res.size);
        }

        let mut child_rects: Vec<Rect> = child_sizes
            .iter()
            .map(|s| Rect {
                x: 0.0,
                y: 0.0,
                width: s.width,
                height: s.height,
            })
            .collect();

        let solver_w = if constraints.max_width.is_finite() {
            constraints.max_width
        } else {
            0.0
        };
        let solver_h = if constraints.max_height.is_finite() {
            constraints.max_height
        } else {
            0.0
        };

        let solver = ConstraintSolver::new(Rect {
            x: 0.0,
            y: 0.0,
            width: solver_w,
            height: solver_h,
        });
        solver.solve(&self.constraints, &mut child_rects);

        let mut max_x = 0.0f32;
        let mut max_y = 0.0f32;
        for r in &child_rects {
            max_x = max_x.max(r.x + r.width);
            max_y = max_y.max(r.y + r.height);
        }

        LayoutResult {
            size: constraints.clamp_size(Size {
                width: max_x,
                height: max_y,
            }),
        }
    }

    fn draw(
        &self,
        rect: Rect,
        draw_list: &mut DrawList,
        text_system: &mut crate::text::TextSystem,
        theme: &crate::theme::Theme,
        focused_id: Option<&str>,
        pointer_pos: Option<crate::types::Point>,
    ) {
        let child_rects = if let Some(id) = self.id() {
            let mut cache = get_or_set_state::<LayoutCache, _>(id, || LayoutCache::default());

            if cache.parent_size != rect.size() || cache.solved_rects.len() != self.children.len() {
                let mut child_sizes = Vec::with_capacity(self.children.len());
                for child in &self.children {
                    let res = child.layout(
                        Constraints::loose(rect.width, rect.height),
                        &[],
                        text_system,
                        theme,
                    );
                    child_sizes.push(res.size);
                }

                let mut solved = child_sizes
                    .iter()
                    .map(|s| Rect {
                        x: 0.0,
                        y: 0.0,
                        width: s.width,
                        height: s.height,
                    })
                    .collect::<Vec<_>>();

                let solver = ConstraintSolver::new(Rect {
                    x: 0.0,
                    y: 0.0,
                    width: rect.width,
                    height: rect.height,
                });
                solver.solve(&self.constraints, &mut solved);

                cache.solved_rects = solved.clone();
                cache.parent_size = rect.size();
                update_state(id, |s| *s = cache.clone());
                solved
            } else {
                cache.solved_rects.clone()
            }
        } else {
            let mut child_sizes = Vec::with_capacity(self.children.len());
            for child in &self.children {
                let res = child.layout(
                    Constraints::loose(rect.width, rect.height),
                    &[],
                    text_system,
                    theme,
                );
                child_sizes.push(res.size);
            }

            let mut solved = child_sizes
                .iter()
                .map(|s| Rect {
                    x: 0.0,
                    y: 0.0,
                    width: s.width,
                    height: s.height,
                })
                .collect::<Vec<_>>();
            let solver = ConstraintSolver::new(Rect {
                x: 0.0,
                y: 0.0,
                width: rect.width,
                height: rect.height,
            });
            solver.solve(&self.constraints, &mut solved);
            solved
        };

        for (i, child) in self.children.iter().enumerate() {
            let mut child_rect = child_rects[i];
            child_rect.x += rect.x;
            child_rect.y += rect.y;
            child.draw(
                child_rect,
                draw_list,
                text_system,
                theme,
                focused_id,
                pointer_pos,
            );
        }
    }

    fn on_event(
        &mut self,
        event: &crate::event::FrameworkEvent,
        rect: Rect,
        text_system: &mut crate::text::TextSystem,
        theme: &crate::theme::Theme,
        focused_id: &mut Option<String>,
        consumed: &mut bool,
    ) -> Vec<M> {
        let child_rects = if let Some(id) = self.id() {
            let cache = get_or_set_state::<LayoutCache, _>(id, || LayoutCache::default());
            if cache.parent_size == rect.size() && cache.solved_rects.len() == self.children.len() {
                cache.solved_rects.clone()
            } else {
                let mut child_sizes = Vec::with_capacity(self.children.len());
                for child in &self.children {
                    let res = child.layout(
                        Constraints::loose(rect.width, rect.height),
                        &[],
                        text_system,
                        theme,
                    );
                    child_sizes.push(res.size);
                }

                let mut solved = child_sizes
                    .iter()
                    .map(|s| Rect {
                        x: 0.0,
                        y: 0.0,
                        width: s.width,
                        height: s.height,
                    })
                    .collect::<Vec<_>>();
                let solver = ConstraintSolver::new(Rect {
                    x: 0.0,
                    y: 0.0,
                    width: rect.width,
                    height: rect.height,
                });
                solver.solve(&self.constraints, &mut solved);
                solved
            }
        } else {
            let mut child_sizes = Vec::with_capacity(self.children.len());
            for child in &self.children {
                let res = child.layout(
                    Constraints::loose(rect.width, rect.height),
                    &[],
                    text_system,
                    theme,
                );
                child_sizes.push(res.size);
            }

            let mut solved = child_sizes
                .iter()
                .map(|s| Rect {
                    x: 0.0,
                    y: 0.0,
                    width: s.width,
                    height: s.height,
                })
                .collect::<Vec<_>>();
            let solver = crate::layout::ConstraintSolver::new(Rect {
                x: 0.0,
                y: 0.0,
                width: rect.width,
                height: rect.height,
            });
            solver.solve(&self.constraints, &mut solved);
            solved
        };

        let mut messages = Vec::new();
        let current_focused = focused_id.clone();

        if let Some(fid) = &current_focused {
            for (i, child) in self.children.iter_mut().enumerate().rev() {
                if child.id() == Some(fid) {
                    let mut child_rect = child_rects[i];
                    child_rect.x += rect.x;
                    child_rect.y += rect.y;
                    messages.extend(child.on_event(
                        event,
                        child_rect,
                        text_system,
                        theme,
                        focused_id,
                        consumed,
                    ));
                    if *consumed {
                        return messages;
                    }
                }
            }
        }

        for (i, child) in self.children.iter_mut().enumerate().rev() {
            let is_focused =
                current_focused.is_some() && child.id().as_deref() == current_focused.as_deref();
            if !is_focused {
                let mut child_rect = child_rects[i];
                child_rect.x += rect.x;
                child_rect.y += rect.y;
                messages.extend(child.on_event(
                    event,
                    child_rect,
                    text_system,
                    theme,
                    focused_id,
                    consumed,
                ));
                if *consumed {
                    return messages;
                }
            }
        }

        messages
    }
}

pub const PARENT: ConstraintTarget = ConstraintTarget::Parent;
pub fn child(idx: usize) -> ConstraintTarget {
    ConstraintTarget::Child(idx)
}
