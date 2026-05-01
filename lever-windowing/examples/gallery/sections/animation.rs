use super::section_card;
use crate::GalleryApp;
use crate::Message;
use lever_core::layout::Alignment;
use lever_core::theme::Theme;
use lever_core::types::Point;
use lever_core::widget::Widget;
use lever_core::widgets::{
    AnimatedOpacity, AnimatedScale, AnimatedTranslation, BoxWidget, Flex, Label, Toggle,
};

pub fn view(app: &GalleryApp, theme: &Theme) -> Box<dyn Widget<Message>> {
    let animated_pulse = lever_core::animated::animated_spring(
        "pulse-scale",
        if app.is_pulsing { 1.1 } else { 1.0 },
        lever_core::animation::Spring::SMOOTH,
    );

    let animated_float = lever_core::animated::animated_spring(
        "float-val",
        if app.is_floating { -20.0 } else { 0.0 },
        lever_core::animation::Spring::SMOOTH,
    );

    let animated_opacity = lever_core::animated::animated_spring(
        "fade-val",
        if app.toggle_on { 1.0 } else { 0.4 },
        lever_core::animation::Spring::SMOOTH,
    );

    section_card(
        "Motion & Animation",
        "Smooth, spring-based transitions for reactive interfaces.",
        Box::new(
            Flex::row(vec![
                Box::new(AnimatedScale::new(
                    animated_pulse,
                    Box::new(
                        BoxWidget::new(theme.primary.with_alpha(0.15))
                            .with_radius(12.0)
                            .with_size(140.0, 100.0)
                            .with_alignment(Alignment::Center)
                            .with_child(Box::new(Label::new("Pulse").with_color(theme.primary))),
                    ),
                )),
                Box::new(AnimatedTranslation::new(
                    Point {
                        x: 0.0,
                        y: animated_float,
                    },
                    Box::new(
                        BoxWidget::new(theme.success.with_alpha(0.15))
                            .with_radius(12.0)
                            .with_size(140.0, 100.0)
                            .with_alignment(Alignment::Center)
                            .with_child(Box::new(Label::new("Float").with_color(theme.success))),
                    ),
                )),
                Box::new(AnimatedOpacity::new(
                    animated_opacity,
                    Box::new(
                        BoxWidget::new(theme.danger.with_alpha(0.15))
                            .with_radius(12.0)
                            .with_size(140.0, 100.0)
                            .with_alignment(Alignment::Center)
                            .with_child(Box::new(Label::new("Fade").with_color(theme.danger))),
                    ),
                )),
                Box::new(
                    Flex::column(vec![
                        Box::new(
                            Flex::row(vec![
                                Box::new(Label::new("Enable Pulse")),
                                Box::new(
                                    Toggle::new("pulse-toggle", app.is_pulsing)
                                        .on_changed(|v| Message::TogglePulse(v)),
                                ),
                            ])
                            .with_gap(12.0),
                        ),
                        Box::new(
                            Flex::row(vec![
                                Box::new(Label::new("Enable Float")),
                                Box::new(
                                    Toggle::new("float-toggle", app.is_floating)
                                        .on_changed(|v| Message::ToggleFloat(v)),
                                ),
                            ])
                            .with_gap(12.0),
                        ),
                        Box::new(
                            Flex::row(vec![
                                Box::new(Label::new("Enable Fade")),
                                Box::new(
                                    Toggle::new("fade-toggle", app.toggle_on)
                                        .on_changed(|v| Message::ToggleChanged(v)),
                                ),
                            ])
                            .with_gap(12.0),
                        ),
                    ])
                    .with_gap(12.0),
                ),
            ])
            .with_gap(24.0),
        ),
        0,
        theme,
    )
}
