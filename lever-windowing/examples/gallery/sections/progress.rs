use super::section_card;
use crate::GalleryApp;
use crate::Message;
use lever_core::theme::Theme;
use lever_core::widget::Widget;
use lever_core::widgets::{CircularProgress, Flex, Label, ProgressBar, Spacer};

pub fn view(app: &GalleryApp, theme: &Theme) -> Box<dyn Widget<Message>> {
    section_card(
        "Progress Indicators",
        "Linear and circular indicators for status and loading states.",
        Box::new(
            Flex::column(vec![
                Box::new(
                    Label::new("Linear Progress")
                        .with_size(12.0)
                        .with_color(theme.text_muted),
                ),
                Box::new(Spacer::new().with_size(10.0, 8.0)),
                Box::new(ProgressBar::new("p1", app.slider_value)),
                Box::new(Spacer::new().with_size(10.0, 24.0)),
                Box::new(
                    Label::new("Indeterminate (Loading)")
                        .with_size(12.0)
                        .with_color(theme.text_muted),
                ),
                Box::new(Spacer::new().with_size(10.0, 8.0)),
                Box::new(ProgressBar::indeterminate("p2").with_color(theme.success)),
                Box::new(Spacer::new().with_size(10.0, 32.0)),
                Box::new(
                    Flex::row(vec![
                        Box::new(
                            Flex::column(vec![
                                Box::new(
                                    Label::new("Circular")
                                        .with_size(12.0)
                                        .with_color(theme.text_muted),
                                ),
                                Box::new(Spacer::new().with_size(10.0, 12.0)),
                                Box::new(CircularProgress::new("c1", app.slider_value)),
                            ])
                            .with_flex(1),
                        ),
                        Box::new(
                            Flex::column(vec![
                                Box::new(
                                    Label::new("Indeterminate")
                                        .with_size(12.0)
                                        .with_color(theme.text_muted),
                                ),
                                Box::new(Spacer::new().with_size(10.0, 12.0)),
                                Box::new(
                                    CircularProgress::indeterminate("c2")
                                        .with_color(theme.secondary),
                                ),
                            ])
                            .with_flex(1),
                        ),
                        Box::new(
                            Flex::column(vec![
                                Box::new(
                                    Label::new("Custom Style")
                                        .with_size(12.0)
                                        .with_color(theme.text_muted),
                                ),
                                Box::new(Spacer::new().with_size(10.0, 12.0)),
                                Box::new(
                                    CircularProgress::new("c3", 0.75)
                                        .with_thickness(8.0)
                                        .with_color(theme.danger),
                                ),
                            ])
                            .with_flex(1),
                        ),
                    ])
                    .with_gap(24.0),
                ),
            ])
            .with_cross_axis_alignment(lever_core::layout::CrossAxisAlignment::Stretch),
        ),
        0,
        theme,
    )
}
