use super::section_card;
use crate::GalleryApp;
use crate::Message;
use lever_core::theme::Theme;
use lever_core::widget::Widget;
use lever_core::widgets::{BoxWidget, Flex, Label, Spacer, SpacerOrientation};

pub fn view(_app: &GalleryApp, theme: &Theme) -> Box<dyn Widget<Message>> {
    Box::new(Flex::column(vec![
        section_card(
            "Typography & Alignment",
            "Consistent sizing and flexible horizontal alignment.",
            Box::new(
                Flex::column(vec![
                    Box::new(
                        Label::new("Left Aligned (Default)")
                            .with_align(lever_core::types::TextAlign::Left),
                    ),
                    Box::new(Spacer::new().with_size(10.0, 12.0)),
                    Box::new(
                        Label::new("Center Aligned Text")
                            .with_align(lever_core::types::TextAlign::Center),
                    ),
                    Box::new(Spacer::new().with_size(10.0, 12.0)),
                    Box::new(
                        Label::new("Right Aligned Text")
                            .with_align(lever_core::types::TextAlign::Right),
                    ),
                    Box::new(Spacer::new().with_size(10.0, 24.0)),
                    Box::new(
                        Flex::row(vec![
                            Box::new(
                                Label::new("Small")
                                    .with_size(theme.font_size_sm)
                                    .with_color(theme.text_muted),
                            ),
                            Box::new(Label::new("Medium").with_size(theme.font_size_md)),
                            Box::new(
                                Label::new("Large")
                                    .with_size(theme.font_size_lg)
                                    .with_color(theme.primary),
                            ),
                        ])
                        .with_gap(24.0)
                        .with_cross_axis_alignment(lever_core::layout::CrossAxisAlignment::Center),
                    ),
                ])
                .with_cross_axis_alignment(lever_core::layout::CrossAxisAlignment::Stretch),
            ),
            0,
            theme,
        ),
        Box::new(Spacer::new().with_size(10.0, 24.0)),
        section_card(
            "Spacing & Dividers",
            "Helpers for layout separation and visual structure.",
            Box::new(Flex::column(vec![
                Box::new(
                    Flex::row(vec![
                        Box::new(Label::new("Item 1")),
                        Box::new(
                            Spacer::horizontal(32.0)
                                .with_divider(theme.border)
                                .with_orientation(SpacerOrientation::Vertical),
                        ),
                        Box::new(Label::new("Item 2")),
                        Box::new(
                            Spacer::horizontal(32.0)
                                .with_divider(theme.primary)
                                .with_thickness(2.0)
                                .with_orientation(SpacerOrientation::Vertical),
                        ),
                        Box::new(Label::new("Item 3")),
                    ])
                    .with_cross_axis_alignment(lever_core::layout::CrossAxisAlignment::Stretch)
                    .with_gap(12.0),
                ),
                Box::new(Spacer::vertical(24.0)),
                Box::new(Flex::row(vec![
                    Box::new(Label::new("Pushed to Left")),
                    Box::new(Spacer::flex()),
                    Box::new(Label::new("Pushed to Right")),
                ])),
                Box::new(
                    Spacer::vertical(32.0)
                        .with_divider(theme.border)
                        .with_orientation(SpacerOrientation::Horizontal),
                ),
                Box::new(
                    Flex::row(vec![
                        Box::new(
                            BoxWidget::new(theme.surface_variant)
                                .with_size(100.0, 100.0)
                                .with_radius(8.0)
                                .with_child(Box::new(Label::new("A").with_size(24.0))),
                        ),
                        Box::new(
                            Spacer::horizontal(32.0)
                                .with_divider(theme.success)
                                .with_thickness(4.0)
                                .with_orientation(SpacerOrientation::Vertical),
                        ),
                        Box::new(
                            BoxWidget::new(theme.surface_variant)
                                .with_size(100.0, 100.0)
                                .with_radius(8.0)
                                .with_child(Box::new(Label::new("B").with_size(24.0))),
                        ),
                    ])
                    .with_cross_axis_alignment(lever_core::layout::CrossAxisAlignment::Stretch)
                    .with_gap(24.0),
                ),
            ])),
            0,
            theme,
        ),
    ]))
}
