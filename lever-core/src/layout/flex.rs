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
    pub gap: f32,
}

impl FlexLayout {
    pub fn new(direction: FlexDirection) -> Self {
        Self {
            direction,
            main_axis_alignment: MainAxisAlignment::Start,
            cross_axis_alignment: CrossAxisAlignment::Start,
            gap: 0.0,
        }
    }

    pub fn layout<M>(
        &self,
        constraints: Constraints,
        children: &[Box<dyn crate::widget::Widget<M>>],
        text_system: &mut crate::text::TextSystem,
        theme: &crate::theme::Theme,
    ) -> (LayoutResult, Vec<Rect>) {
        let mut child_results = vec![
            LayoutResult {
                size: Size {
                    width: 0.0,
                    height: 0.0
                }
            };
            children.len()
        ];
        let mut total_flex = 0;
        let mut used_main = 0.0;
        let mut max_cross: f32 = 0.0;

        for (i, child) in children.iter().enumerate() {
            let flex = child.flex();
            if flex > 0 {
                total_flex += flex;
            } else {
                let res = child.layout(
                    Constraints::loose(constraints.max_width, constraints.max_height),
                    &[],
                    text_system,
                    theme,
                );
                child_results[i] = res;
                match self.direction {
                    FlexDirection::Row => {
                        used_main += res.size.width;
                        max_cross = max_cross.max(res.size.height);
                    }
                    FlexDirection::Column => {
                        used_main += res.size.height;
                        max_cross = max_cross.max(res.size.width);
                    }
                }
            }
        }

        if children.len() > 1 {
            used_main += self.gap * (children.len() - 1) as f32;
        }

        if total_flex > 0 {
            let available_main = match self.direction {
                FlexDirection::Row => constraints.max_width,
                FlexDirection::Column => constraints.max_height,
            };
            let remaining_main = (available_main - used_main).max(0.0);
            let main_per_flex = remaining_main / total_flex as f32;

            for (i, child) in children.iter().enumerate() {
                let flex = child.flex();
                if flex > 0 {
                    let child_main = main_per_flex * flex as f32;
                    let child_constraints = match self.direction {
                        FlexDirection::Row => {
                            Constraints::tight(child_main, constraints.max_height)
                        }
                        FlexDirection::Column => {
                            Constraints::tight(constraints.max_width, child_main)
                        }
                    };
                    let res = child.layout(child_constraints, &[], text_system, theme);
                    child_results[i] = res;
                    match self.direction {
                        FlexDirection::Row => max_cross = max_cross.max(res.size.height),
                        FlexDirection::Column => max_cross = max_cross.max(res.size.width),
                    }
                }
            }
        }

        let final_main = if total_flex > 0 {
            match self.direction {
                FlexDirection::Row => constraints.max_width,
                FlexDirection::Column => constraints.max_height,
            }
        } else {
            used_main
        };

        let final_size = constraints.clamp_size(match self.direction {
            FlexDirection::Row => Size {
                width: final_main,
                height: max_cross,
            },
            FlexDirection::Column => Size {
                width: max_cross,
                height: final_main,
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
                    current_main += res.size.width + self.gap;
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
                    current_main += res.size.height + self.gap;
                    r
                }
            };
            child_rects.push(rect);
        }

        (LayoutResult { size: final_size }, child_rects)
    }
}
