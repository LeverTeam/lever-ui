use super::section_card;
use crate::GalleryApp;
use crate::Message;
use lever_core::theme::Theme;
use lever_core::types::ImageFit;
use lever_core::widget::Widget;
use lever_core::widgets::{BoxWidget, Flex, ImageWidget, Label, Spacer};

pub fn view(app: &GalleryApp, theme: &Theme) -> Box<dyn Widget<Message>> {
    let test_tex = app.test_image.unwrap_or(lever_core::types::TextureId(0));

    section_card(
        "Images & Media",
        "High-performance image rendering with multiple fit modes.",
        Box::new(
            Flex::row(vec![
                Box::new(
                    Flex::column(vec![
                        Box::new(
                            Label::new("Fill")
                                .with_size(12.0)
                                .with_color(theme.text_muted),
                        ),
                        Box::new(Spacer::new().with_size(10.0, 8.0)),
                        Box::new(
                            BoxWidget::new(theme.surface_variant)
                                .with_radius(8.0)
                                .with_size(120.0, 120.0)
                                .with_child(Box::new(
                                    ImageWidget::new(test_tex)
                                        .with_size(120.0, 120.0)
                                        .with_fit(ImageFit::Fill),
                                )),
                        ),
                    ])
                    .with_cross_axis_alignment(lever_core::layout::CrossAxisAlignment::Center),
                ),
                Box::new(
                    Flex::column(vec![
                        Box::new(
                            Label::new("Contain")
                                .with_size(12.0)
                                .with_color(theme.text_muted),
                        ),
                        Box::new(Spacer::new().with_size(10.0, 8.0)),
                        Box::new(
                            BoxWidget::new(theme.surface_variant)
                                .with_radius(8.0)
                                .with_size(120.0, 120.0)
                                .with_child(Box::new(
                                    ImageWidget::new(test_tex)
                                        .with_size(120.0, 120.0)
                                        .with_fit(ImageFit::Contain),
                                )),
                        ),
                    ])
                    .with_cross_axis_alignment(lever_core::layout::CrossAxisAlignment::Center),
                ),
                Box::new(
                    Flex::column(vec![
                        Box::new(
                            Label::new("Cover")
                                .with_size(12.0)
                                .with_color(theme.text_muted),
                        ),
                        Box::new(Spacer::new().with_size(10.0, 8.0)),
                        Box::new(
                            BoxWidget::new(theme.surface_variant)
                                .with_radius(8.0)
                                .with_size(120.0, 120.0)
                                .with_child(Box::new(
                                    ImageWidget::new(test_tex)
                                        .with_size(120.0, 120.0)
                                        .with_fit(ImageFit::Cover),
                                )),
                        ),
                    ])
                    .with_cross_axis_alignment(lever_core::layout::CrossAxisAlignment::Center),
                ),
            ])
            .with_gap(24.0),
        ),
        0,
        theme,
    )
}
