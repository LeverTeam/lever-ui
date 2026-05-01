use crate::draw::DrawList;
use crate::event::FrameworkEvent;
use crate::layout::{Constraints, LayoutNode, LayoutResult};
use crate::types::{Rect, Size};
use crate::widget::Widget;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum WrapDirection {
    #[default]
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum WrapAlignment {
    #[default]
    Start,
    Center,
    End,
}

pub struct Wrap<M> {
    pub children: Vec<Box<dyn Widget<M>>>,
    pub direction: WrapDirection,
    pub spacing: f32,
    pub run_spacing: f32,
    pub alignment: WrapAlignment,
    pub run_alignment: WrapAlignment,
    pub cross_alignment: WrapAlignment,
}

impl<M> Wrap<M> {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            direction: WrapDirection::Horizontal,
            spacing: 0.0,
            run_spacing: 0.0,
            alignment: WrapAlignment::Start,
            run_alignment: WrapAlignment::Start,
            cross_alignment: WrapAlignment::Start,
        }
    }

    pub fn with_child(mut self, child: Box<dyn Widget<M>>) -> Self {
        self.children.push(child);
        self
    }

    pub fn with_children(mut self, children: Vec<Box<dyn Widget<M>>>) -> Self {
        self.children.extend(children);
        self
    }

    pub fn with_spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    pub fn with_run_spacing(mut self, spacing: f32) -> Self {
        self.run_spacing = spacing;
        self
    }

    pub fn with_direction(mut self, direction: WrapDirection) -> Self {
        self.direction = direction;
        self
    }

    fn calculate_layout(
        &self,
        constraints: Constraints,
        text_system: &mut crate::text::TextSystem,
        theme: &crate::theme::Theme,
    ) -> (Size, Vec<Vec<(usize, Size)>>) {
        let mut runs = Vec::new();
        let mut current_run = Vec::new();
        let mut current_run_size = 0.0f32;
        let mut current_run_cross_max = 0.0f32;

        let max_main = match self.direction {
            WrapDirection::Horizontal => constraints.max_width,
            WrapDirection::Vertical => constraints.max_height,
        };

        for (idx, child) in self.children.iter().enumerate() {
            let res = child.layout(
                Constraints::loose(constraints.max_width, constraints.max_height),
                &[],
                text_system,
                theme,
            );

            let child_main = match self.direction {
                WrapDirection::Horizontal => res.size.width,
                WrapDirection::Vertical => res.size.height,
            };
            let child_cross = match self.direction {
                WrapDirection::Horizontal => res.size.height,
                WrapDirection::Vertical => res.size.width,
            };

            let spacing = if current_run.is_empty() {
                0.0
            } else {
                self.spacing
            };

            if current_run_size + spacing + child_main > max_main && !current_run.is_empty() {
                runs.push(current_run);
                current_run = Vec::new();
                current_run_size = 0.0;
                current_run_cross_max = 0.0;
            }

            let actual_spacing = if current_run.is_empty() {
                0.0
            } else {
                self.spacing
            };
            current_run_size += actual_spacing + child_main;
            current_run_cross_max = current_run_cross_max.max(child_cross);
            current_run.push((idx, res.size));
        }

        if !current_run.is_empty() {
            runs.push(current_run);
        }

        let mut total_main = 0.0f32;
        let mut total_cross = 0.0f32;

        for (i, run) in runs.iter().enumerate() {
            let mut run_main = 0.0f32;
            let mut run_cross = 0.0f32;
            for (_, size) in run {
                let m = match self.direction {
                    WrapDirection::Horizontal => size.width,
                    WrapDirection::Vertical => size.height,
                };
                let c = match self.direction {
                    WrapDirection::Horizontal => size.height,
                    WrapDirection::Vertical => size.width,
                };
                run_main += m;
                run_cross = run_cross.max(c);
            }
            run_main += (run.len().saturating_sub(1) as f32) * self.spacing;
            total_main = total_main.max(run_main);
            total_cross += run_cross;
            if i > 0 {
                total_cross += self.run_spacing;
            }
        }

        let size = match self.direction {
            WrapDirection::Horizontal => Size {
                width: total_main.clamp(constraints.min_width, constraints.max_width),
                height: total_cross.clamp(constraints.min_height, constraints.max_height),
            },
            WrapDirection::Vertical => Size {
                width: total_cross.clamp(constraints.min_width, constraints.max_width),
                height: total_main.clamp(constraints.min_height, constraints.max_height),
            },
        };

        (size, runs)
    }
}

impl<M: 'static> Widget<M> for Wrap<M> {
    fn layout(
        &self,
        constraints: Constraints,
        _children: &[LayoutNode],
        text_system: &mut crate::text::TextSystem,
        theme: &crate::theme::Theme,
    ) -> LayoutResult {
        let (size, _) = self.calculate_layout(constraints, text_system, theme);
        LayoutResult { size }
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
        let constraints = Constraints::tight(rect.width, rect.height);
        let (_, runs) = self.calculate_layout(constraints, text_system, theme);

        let mut current_cross = 0.0f32;
        for run in runs {
            let mut run_main_total = 0.0f32;
            let mut run_cross_max = 0.0f32;
            for (_, size) in &run {
                run_main_total += match self.direction {
                    WrapDirection::Horizontal => size.width,
                    WrapDirection::Vertical => size.height,
                };
                run_cross_max = run_cross_max.max(match self.direction {
                    WrapDirection::Horizontal => size.height,
                    WrapDirection::Vertical => size.width,
                });
            }
            run_main_total += (run.len().saturating_sub(1) as f32) * self.spacing;

            let mut current_main = match self.alignment {
                WrapAlignment::Start => 0.0,
                WrapAlignment::Center => {
                    (match self.direction {
                        WrapDirection::Horizontal => rect.width,
                        WrapDirection::Vertical => rect.height,
                    } - run_main_total)
                        / 2.0
                }
                WrapAlignment::End => {
                    (match self.direction {
                        WrapDirection::Horizontal => rect.width,
                        WrapDirection::Vertical => rect.height,
                    }) - run_main_total
                }
            };

            for (idx, size) in run {
                let child_main = match self.direction {
                    WrapDirection::Horizontal => size.width,
                    WrapDirection::Vertical => size.height,
                };
                let child_cross = match self.direction {
                    WrapDirection::Horizontal => size.height,
                    WrapDirection::Vertical => size.width,
                };

                let cross_offset = match self.cross_alignment {
                    WrapAlignment::Start => 0.0,
                    WrapAlignment::Center => (run_cross_max - child_cross) / 2.0,
                    WrapAlignment::End => run_cross_max - child_cross,
                };

                let child_rect = match self.direction {
                    WrapDirection::Horizontal => Rect {
                        x: rect.x + current_main,
                        y: rect.y + current_cross + cross_offset,
                        width: size.width,
                        height: size.height,
                    },
                    WrapDirection::Vertical => Rect {
                        x: rect.x + current_cross + cross_offset,
                        y: rect.y + current_main,
                        width: size.width,
                        height: size.height,
                    },
                };

                self.children[idx].draw(
                    child_rect,
                    draw_list,
                    text_system,
                    theme,
                    focused_id,
                    pointer_pos,
                );

                current_main += child_main + self.spacing;
            }

            current_cross += run_cross_max + self.run_spacing;
        }
    }

    fn on_event(
        &mut self,
        event: &FrameworkEvent,
        rect: Rect,
        text_system: &mut crate::text::TextSystem,
        theme: &crate::theme::Theme,
        focused_id: &mut Option<String>,
        consumed: &mut bool,
    ) -> Vec<M> {
        let mut messages = Vec::new();
        let constraints = Constraints::tight(rect.width, rect.height);
        let (_, runs) = self.calculate_layout(constraints, text_system, theme);

        let mut current_cross = 0.0f32;
        for run in runs {
            let mut run_main_total = 0.0f32;
            let mut run_cross_max = 0.0f32;
            for (_, size) in &run {
                run_main_total += match self.direction {
                    WrapDirection::Horizontal => size.width,
                    WrapDirection::Vertical => size.height,
                };
                run_cross_max = run_cross_max.max(match self.direction {
                    WrapDirection::Horizontal => size.height,
                    WrapDirection::Vertical => size.width,
                });
            }
            run_main_total += (run.len().saturating_sub(1) as f32) * self.spacing;

            let mut current_main = match self.alignment {
                WrapAlignment::Start => 0.0,
                WrapAlignment::Center => {
                    (match self.direction {
                        WrapDirection::Horizontal => rect.width,
                        WrapDirection::Vertical => rect.height,
                    } - run_main_total)
                        / 2.0
                }
                WrapAlignment::End => {
                    (match self.direction {
                        WrapDirection::Horizontal => rect.width,
                        WrapDirection::Vertical => rect.height,
                    }) - run_main_total
                }
            };

            for (idx, size) in run {
                let child_main = match self.direction {
                    WrapDirection::Horizontal => size.width,
                    WrapDirection::Vertical => size.height,
                };
                let child_cross = match self.direction {
                    WrapDirection::Horizontal => size.height,
                    WrapDirection::Vertical => size.width,
                };

                let cross_offset = match self.cross_alignment {
                    WrapAlignment::Start => 0.0,
                    WrapAlignment::Center => (run_cross_max - child_cross) / 2.0,
                    WrapAlignment::End => run_cross_max - child_cross,
                };

                let child_rect = match self.direction {
                    WrapDirection::Horizontal => Rect {
                        x: rect.x + current_main,
                        y: rect.y + current_cross + cross_offset,
                        width: size.width,
                        height: size.height,
                    },
                    WrapDirection::Vertical => Rect {
                        x: rect.x + current_cross + cross_offset,
                        y: rect.y + current_main,
                        width: size.width,
                        height: size.height,
                    },
                };

                messages.extend(self.children[idx].on_event(
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

                current_main += child_main + self.spacing;
            }

            current_cross += run_cross_max + self.run_spacing;
        }

        messages
    }
}
