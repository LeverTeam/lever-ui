use crate::layout::{Constraints, LayoutResult};
use crate::types::{Rect, Size};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FlexDirection {
    Row,
    Column,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MainAxisAlignment {
    Start,
    Center,
    End,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CrossAxisAlignment {
    Start,
    Center,
    End,
    Stretch,
}

pub struct FlexLayout {
    pub direction: FlexDirection,
    pub main_axis_alignment: MainAxisAlignment,
    pub cross_axis_alignment: CrossAxisAlignment,
    pub spacing: f32,
}

impl FlexLayout {
    pub fn new(direction: FlexDirection) -> Self {
        Self {
            direction,
            main_axis_alignment: MainAxisAlignment::Start,
            cross_axis_alignment: CrossAxisAlignment::Start,
            spacing: 0.0,
        }
    }

    pub fn layout(
        &self,
        constraints: Constraints,
        children: &[Box<dyn crate::widget::Widget>],
        text_system: &mut crate::text::TextSystem,
        theme: &crate::theme::Theme,
    ) -> (LayoutResult, Vec<Rect>) {
        let mut child_results = Vec::with_capacity(children.len());
        let mut total_main = 0.0;
        let mut max_cross: f32 = 0.0;

        let child_constraints = match self.direction {
            FlexDirection::Row => Constraints::loose(constraints.max_width, constraints.max_height),
            FlexDirection::Column => {
                Constraints::loose(constraints.max_width, constraints.max_height)
            }
        };

        for child in children {
            let res = child.layout(child_constraints, &[], text_system, theme);
            child_results.push(res);

            match self.direction {
                FlexDirection::Row => {
                    total_main += res.size.width;
                    max_cross = max_cross.max(res.size.height);
                }
                FlexDirection::Column => {
                    total_main += res.size.height;
                    max_cross = max_cross.max(res.size.width);
                }
            }
        }

        if !children.is_empty() {
            total_main += self.spacing * (children.len() - 1) as f32;
        }

        let final_size = constraints.clamp_size(match self.direction {
            FlexDirection::Row => Size {
                width: total_main,
                height: max_cross,
            },
            FlexDirection::Column => Size {
                width: max_cross,
                height: total_main,
            },
        });

        let mut child_rects = Vec::with_capacity(children.len());
        let mut current_main = 0.0;

        for res in child_results {
            let rect = match self.direction {
                FlexDirection::Row => {
                    let y = match self.cross_axis_alignment {
                        CrossAxisAlignment::Start => 0.0,
                        CrossAxisAlignment::Center => (final_size.height - res.size.height) / 2.0,
                        CrossAxisAlignment::End => final_size.height - res.size.height,
                        CrossAxisAlignment::Stretch => 0.0,
                    };
                    let r = Rect {
                        x: current_main,
                        y,
                        width: res.size.width,
                        height: if self.cross_axis_alignment == CrossAxisAlignment::Stretch {
                            final_size.height
                        } else {
                            res.size.height
                        },
                    };
                    current_main += res.size.width + self.spacing;
                    r
                }
                FlexDirection::Column => {
                    let x = match self.cross_axis_alignment {
                        CrossAxisAlignment::Start => 0.0,
                        CrossAxisAlignment::Center => (final_size.width - res.size.width) / 2.0,
                        CrossAxisAlignment::End => final_size.width - res.size.width,
                        CrossAxisAlignment::Stretch => 0.0,
                    };
                    let r = Rect {
                        x,
                        y: current_main,
                        width: if self.cross_axis_alignment == CrossAxisAlignment::Stretch {
                            final_size.width
                        } else {
                            res.size.width
                        },
                        height: res.size.height,
                    };
                    current_main += res.size.height + self.spacing;
                    r
                }
            };
            child_rects.push(rect);
        }

        (LayoutResult { size: final_size }, child_rects)
    }
}
