use super::section_card;
use crate::GalleryApp;
use crate::Message;
use lever_core::layout::Alignment;
use lever_core::theme::Theme;
use lever_core::types::Color;
use lever_core::widget::Widget;
use lever_core::widgets::{BoxWidget, Flex, Label, Spacer, Stack, Wrap};

pub fn view(_app: &GalleryApp, theme: &Theme) -> Box<dyn Widget<Message>> {
    Box::new(Flex::column(vec![
        section_card(
            "Stack Layout",
            "Layers multiple widgets on top of each other.",
            Box::new(
                Stack::new()
                    .with_alignment(Alignment::Center)
                    .with_child(Box::new(
                        BoxWidget::new(theme.primary.with_alpha(0.2))
                            .with_radius(12.0)
                            .with_size(200.0, 200.0),
                    ))
                    .with_child(Box::new(
                        BoxWidget::new(theme.success.with_alpha(0.4))
                            .with_radius(12.0)
                            .with_size(140.0, 140.0),
                    ))
                    .with_child(Box::new(
                        BoxWidget::new(theme.danger.with_alpha(0.6))
                            .with_radius(12.0)
                            .with_size(80.0, 80.0),
                    ))
                    .with_child(Box::new(
                        Label::new("STACK").with_size(14.0).with_color(Color::WHITE),
                    )),
            ),
            0,
            theme,
        ),
        Box::new(Spacer::vertical(24.0)),
        section_card(
            "Wrap / Flow Layout",
            "Automatically wraps items to the next line when space is limited.",
            Box::new(
                Wrap::new()
                    .with_spacing(8.0)
                    .with_run_spacing(8.0)
                    .with_children(
                        vec![
                            "Lever",
                            "UI",
                            "Framework",
                            "Rust",
                            "OpenGL",
                            "Modern",
                            "Fast",
                            "Layout",
                            "Widgets",
                            "Animation",
                            "Theming",
                            "Cross-Platform",
                            "High-Performance",
                            "Declarative",
                            "Reactive",
                        ]
                        .into_iter()
                        .map(|s| {
                            Box::new(
                                BoxWidget::new(theme.primary.with_alpha(0.1))
                                    .with_radius(20.0)
                                    .with_padding(lever_core::types::SideOffsets::new(
                                        6.0, 16.0, 6.0, 16.0,
                                    ))
                                    .with_child(Box::new(
                                        Label::new(s).with_size(13.0).with_color(theme.primary),
                                    )),
                            ) as Box<dyn Widget<Message>>
                        })
                        .collect(),
                    ),
            ),
            0,
            theme,
        ),
    ]))
}
