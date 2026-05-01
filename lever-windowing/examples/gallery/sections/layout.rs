use super::section_card;
use crate::GalleryApp;
use crate::Message;
use lever_core::layout::GridTrack;
use lever_core::theme::Theme;
use lever_core::types::Color;
use lever_core::widget::Widget;
use lever_core::widgets::{child, BoxWidget, ConstraintLayout, Flex, Grid, PARENT};

pub fn view(_app: &GalleryApp, theme: &Theme) -> Box<dyn Widget<Message>> {
    Box::new(
        Flex::row(vec![
            section_card(
                "Constraint Layout",
                "Powerful relative positioning system.",
                Box::new(
                    BoxWidget::new(Color::TRANSPARENT)
                        .with_size(0.0, 160.0)
                        .with_child(Box::new(
                            ConstraintLayout::new()
                                .with_id("demo-cl")
                                .with_child(
                                    Box::new(
                                        BoxWidget::new(theme.primary)
                                            .with_radius(8.0)
                                            .with_size(60.0, 60.0),
                                    ),
                                    |set| set.center_x(PARENT, 0.0).center_y(PARENT, 0.0),
                                )
                                .with_child(
                                    Box::new(
                                        BoxWidget::new(theme.success.with_alpha(0.8))
                                            .with_radius(8.0)
                                            .with_size(40.0, 40.0),
                                    ),
                                    |set| set.right_to_left(child(0), -12.0).center_y(PARENT, 0.0),
                                )
                                .with_child(
                                    Box::new(
                                        BoxWidget::new(theme.danger.with_alpha(0.8))
                                            .with_radius(8.0)
                                            .with_size(40.0, 40.0),
                                    ),
                                    |set| set.left_to_right(child(0), 12.0).center_y(PARENT, 0.0),
                                )
                                .with_child(
                                    Box::new(
                                        BoxWidget::new(theme.secondary.with_alpha(0.8))
                                            .with_radius(8.0)
                                            .with_size(120.0, 32.0),
                                    ),
                                    |set| set.top_to_bottom(child(0), 12.0).center_x(PARENT, 0.0),
                                ),
                        )),
                ),
                1,
                theme,
            ),
            section_card(
                "Grid System",
                "Flexible track-based alignment.",
                Box::new(
                    BoxWidget::new(Color::TRANSPARENT)
                        .with_size(0.0, 160.0)
                        .with_child(Box::new(
                            Grid::new()
                                .with_column(GridTrack::Flex(1))
                                .with_column(GridTrack::Flex(1))
                                .with_child(Box::new(
                                    BoxWidget::new(theme.primary.with_alpha(0.2))
                                        .with_radius(6.0)
                                        .with_size(0.0, 60.0),
                                ))
                                .with_child(Box::new(
                                    BoxWidget::new(theme.success.with_alpha(0.2))
                                        .with_radius(6.0)
                                        .with_size(0.0, 60.0),
                                ))
                                .with_child(Box::new(
                                    BoxWidget::new(theme.danger.with_alpha(0.2))
                                        .with_radius(6.0)
                                        .with_size(0.0, 60.0),
                                ))
                                .with_child(Box::new(
                                    BoxWidget::new(theme.secondary.with_alpha(0.2))
                                        .with_radius(6.0)
                                        .with_size(0.0, 60.0),
                                ))
                                .with_gap(12.0),
                        )),
                ),
                1,
                theme,
            ),
        ])
        .with_gap(24.0),
    )
}
