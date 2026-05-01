use super::section_card;
use crate::GalleryApp;
use crate::Message;
use lever_core::theme::Theme;
use lever_core::widget::Widget;
use lever_core::widgets::{
    Button, ButtonVariant, Checkbox, Dropdown, Flex, Label, RadioGroup, RadioOption, Slider, Spacer,
};

pub fn view(app: &GalleryApp, theme: &Theme) -> Box<dyn Widget<Message>> {
    section_card(
        "Interactive Components",
        "Essential building blocks for user input and actions.",
        Box::new(Flex::column(vec![
            Box::new(
                Flex::row(vec![
                    Box::new(
                        Button::new("btn-p", "Primary Action")
                            .with_variant(ButtonVariant::Primary)
                            .on_click(|| Message::ButtonClicked("P".into())),
                    ),
                    Box::new(
                        Button::new("btn-s", "Secondary").with_variant(ButtonVariant::Secondary),
                    ),
                    Box::new(Button::new("btn-o", "Outline").with_variant(ButtonVariant::Outline)),
                    Box::new(
                        Button::new("btn-modal", "Show Modal")
                            .with_variant(ButtonVariant::Primary)
                            .on_click(|| Message::OpenModal),
                    ),
                ])
                .with_gap(12.0),
            ),
            Box::new(Spacer::new().with_size(10.0, 20.0)),
            Box::new(
                Flex::row(vec![
                    Box::new(
                        Checkbox::new("c1", app.checkbox_checked)
                            .with_label("Agree to terms")
                            .on_changed(|v| Message::CheckboxChanged(v)),
                    ),
                    Box::new(
                        Checkbox::new("c2", !app.checkbox_checked).with_label("Opt-in to news"),
                    ),
                ])
                .with_gap(32.0),
            ),
            Box::new(Spacer::new().with_size(10.0, 20.0)),
            Box::new(
                Flex::row(vec![
                    Box::new(
                        Dropdown::new(
                            "g-drop",
                            vec![
                                "Standard Mode".into(),
                                "Compact Mode".into(),
                                "Experimental".into(),
                                "Debug".into(),
                            ],
                            app.selected_dropdown,
                        )
                        .on_select(|idx| Message::DropdownSelected(idx)),
                    ),
                    Box::new(
                        RadioGroup::new(
                            "radio-group-demo",
                            vec![
                                RadioOption::new("Option A", 0),
                                RadioOption::new("Option B", 1),
                                RadioOption::new("Disabled Option", 2).with_disabled(true),
                            ],
                        )
                        .with_selected(app.selected_radio)
                        .with_direction(lever_core::layout::FlexDirection::Row)
                        .with_gap(24.0)
                        .on_changed(|v| Message::RadioSelected(v)),
                    ),
                ])
                .with_gap(32.0),
            ),
            Box::new(Spacer::new().with_size(10.0, 32.0)),
            Box::new(Flex::column(vec![
                Box::new(Label::new("Sliders").with_size(16.0)),
                Box::new(Spacer::new().with_size(10.0, 16.0)),
                Box::new(
                    Flex::row(vec![
                        Box::new(
                            Flex::column(vec![
                                Box::new(
                                    Label::new("Continuous (0-1)")
                                        .with_size(12.0)
                                        .with_color(theme.text_muted),
                                ),
                                Box::new(Spacer::new().with_size(10.0, 8.0)),
                                Box::new(
                                    Slider::new("s1", app.slider_value)
                                        .on_changed(|v| Message::SliderChanged(v)),
                                ),
                            ])
                            .with_flex(1),
                        ),
                        Box::new(
                            Flex::column(vec![
                                Box::new(
                                    Label::new("Discrete (0-100, step 10)")
                                        .with_size(12.0)
                                        .with_color(theme.text_muted),
                                ),
                                Box::new(Spacer::new().with_size(10.0, 8.0)),
                                Box::new(
                                    Slider::new("s2", app.discrete_slider)
                                        .with_range(0.0, 100.0)
                                        .with_step(10.0)
                                        .on_changed(|v| Message::DiscreteSliderChanged(v)),
                                ),
                            ])
                            .with_flex(1),
                        ),
                    ])
                    .with_gap(24.0),
                ),
                Box::new(Spacer::new().with_size(10.0, 24.0)),
                Box::new(
                    Flex::row(vec![
                        Box::new(
                            Flex::column(vec![
                                Box::new(
                                    Label::new("Percent Formatter")
                                        .with_size(12.0)
                                        .with_color(theme.text_muted),
                                ),
                                Box::new(Spacer::new().with_size(10.0, 8.0)),
                                Box::new(
                                    Slider::new("s3", app.percent_slider)
                                        .with_label_formatter(|v| format!("{:.0}%", v * 100.0))
                                        .on_changed(|v| Message::PercentSliderChanged(v)),
                                ),
                            ])
                            .with_flex(1),
                        ),
                        Box::new(
                            Flex::column(vec![
                                Box::new(
                                    Label::new("Disabled State")
                                        .with_size(12.0)
                                        .with_color(theme.text_muted),
                                ),
                                Box::new(Spacer::new().with_size(10.0, 8.0)),
                                Box::new(Slider::new("s4", 0.3).with_disabled(true)),
                            ])
                            .with_flex(1),
                        ),
                    ])
                    .with_gap(24.0),
                ),
            ])),
        ])),
        0,
        theme,
    )
}
