use crate::draw::DrawList;
use crate::layout::{Alignment, Constraints, LayoutNode, LayoutResult};
use crate::types::{Rect, Size};
use crate::widget::Widget;

pub struct Stack<M> {
    pub id: Option<String>,
    pub children: Vec<Box<dyn Widget<M>>>,
    pub alignment: Alignment,
}

impl<M> Stack<M> {
    pub fn new(children: Vec<Box<dyn Widget<M>>>) -> Self {
        Self {
            id: None,
            children,
            alignment: Alignment::default(),
        }
    }

    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn with_alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }

    fn calculate_child_rects(&self, rect: Rect, child_sizes: &[Size]) -> Vec<Rect> {
        let mut child_rects = Vec::with_capacity(self.children.len());

        for (i, child) in self.children.iter().enumerate() {
            let child_size = child_sizes[i];

            if let Some(pos) = child.positioned() {
                let mut x = rect.x;
                let mut y = rect.y;
                let mut width = child_size.width;
                let mut height = child_size.height;

                if let Some(left) = pos.left {
                    x += left;
                    if let Some(right) = pos.right {
                        width = (rect.width - left - right).max(0.0);
                    }
                } else if let Some(right) = pos.right {
                    x += (rect.width - child_size.width - right).max(0.0);
                } else {
                    let (dx, _) = self.alignment.align(child_size, rect.size());
                    x += dx;
                }

                if let Some(top) = pos.top {
                    y += top;
                    if let Some(bottom) = pos.bottom {
                        height = (rect.height - top - bottom).max(0.0);
                    }
                } else if let Some(bottom) = pos.bottom {
                    y += (rect.height - child_size.height - bottom).max(0.0);
                } else {
                    let (_, dy) = self.alignment.align(child_size, rect.size());
                    y += dy;
                }

                child_rects.push(Rect {
                    x,
                    y,
                    width,
                    height,
                });
            } else {
                let (dx, dy) = self.alignment.align(child_size, rect.size());
                child_rects.push(Rect {
                    x: rect.x + dx,
                    y: rect.y + dy,
                    width: child_size.width,
                    height: child_size.height,
                });
            }
        }

        child_rects
    }
}

impl<M: 'static> Widget<M> for Stack<M> {
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
        let mut max_width: f32 = 0.0;
        let mut max_height: f32 = 0.0;

        for child in &self.children {
            let child_constraints = if let Some(pos) = child.positioned() {
                let parent_max_w = if constraints.max_width.is_finite() {
                    constraints.max_width
                } else {
                    10000.0
                };
                let parent_max_h = if constraints.max_height.is_finite() {
                    constraints.max_height
                } else {
                    10000.0
                };

                let min_w = if let (Some(l), Some(r)) = (pos.left, pos.right) {
                    (parent_max_w - l - r).max(0.0)
                } else {
                    0.0
                };
                let min_h = if let (Some(t), Some(b)) = (pos.top, pos.bottom) {
                    (parent_max_h - t - b).max(0.0)
                } else {
                    0.0
                };

                Constraints {
                    min_width: min_w,
                    max_width: if min_w > 0.0 {
                        min_w
                    } else {
                        constraints.max_width
                    },
                    min_height: min_h,
                    max_height: if min_h > 0.0 {
                        min_h
                    } else {
                        constraints.max_height
                    },
                }
            } else {
                Constraints::loose(constraints.max_width, constraints.max_height)
            };

            let res = child.layout(child_constraints, &[], text_system, theme);
            max_width = max_width.max(res.size.width);
            max_height = max_height.max(res.size.height);
        }

        LayoutResult {
            size: constraints.clamp_size(Size {
                width: max_width,
                height: max_height,
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
        let mut child_sizes = Vec::with_capacity(self.children.len());
        for child in &self.children {
            let child_constraints = if let Some(pos) = child.positioned() {
                let min_w = if let (Some(l), Some(r)) = (pos.left, pos.right) {
                    (rect.width - l - r).max(0.0)
                } else {
                    0.0
                };
                let min_h = if let (Some(t), Some(b)) = (pos.top, pos.bottom) {
                    (rect.height - t - b).max(0.0)
                } else {
                    0.0
                };
                Constraints {
                    min_width: min_w,
                    max_width: if min_w > 0.0 { min_w } else { rect.width },
                    min_height: min_h,
                    max_height: if min_h > 0.0 { min_h } else { rect.height },
                }
            } else {
                Constraints::loose(rect.width, rect.height)
            };

            let res = child.layout(child_constraints, &[], text_system, theme);
            child_sizes.push(res.size);
        }

        let child_rects = self.calculate_child_rects(rect, &child_sizes);

        for (i, child) in self.children.iter().enumerate() {
            child.draw(
                child_rects[i],
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
    ) -> Vec<M> {
        let mut child_sizes = Vec::with_capacity(self.children.len());
        for child in &self.children {
            let child_constraints = if let Some(pos) = child.positioned() {
                let min_w = if let (Some(l), Some(r)) = (pos.left, pos.right) {
                    (rect.width - l - r).max(0.0)
                } else {
                    0.0
                };
                let min_h = if let (Some(t), Some(b)) = (pos.top, pos.bottom) {
                    (rect.height - t - b).max(0.0)
                } else {
                    0.0
                };
                Constraints {
                    min_width: min_w,
                    max_width: if min_w > 0.0 { min_w } else { rect.width },
                    min_height: min_h,
                    max_height: if min_h > 0.0 { min_h } else { rect.height },
                }
            } else {
                Constraints::loose(rect.width, rect.height)
            };

            let res = child.layout(child_constraints, &[], text_system, theme);
            child_sizes.push(res.size);
        }

        let child_rects = self.calculate_child_rects(rect, &child_sizes);

        let mut messages = Vec::new();
        for (i, child) in self.children.iter_mut().enumerate().rev() {
            let res = child.on_event(event, child_rects[i], text_system, theme, focused_id);
            if !res.is_empty() {
                messages.extend(res);
                return messages;
            }
        }
        messages
    }
}
