use lever_core::app::{App, UpdateContext};
use lever_core::layout::{Alignment, GridTrack};
use lever_core::theme::ThemeMode;
use lever_core::types::{Color, Point, SideOffsets};
use lever_core::widgets::{
    AnimatedOpacity, AnimatedScale, AnimatedTranslation, BoxWidget, Button, ButtonSize,
    ButtonVariant, Flex, Grid, Label, Spacer, ThemeToggle, Toggle,
};
use lever_windowing::application::Application;
use lever_windowing::config::AppConfig;

#[derive(Debug, Clone)]
pub enum Message {
    TextChanged(String, usize),
    ButtonClicked(String),
    ToggleChanged(bool),
    SliderChanged(f32),
    CheckboxChanged(bool),
    ThemeModeChanged(ThemeMode),
    TogglePulse(bool),
    ToggleFloat(bool),
}

struct GalleryApp {
    input_text: String,
    cursor_index: usize,
    toggle_on: bool,
    slider_value: f32,
    checkbox_checked: bool,
    theme_mode: ThemeMode,
    is_pulsing: bool,
    is_floating: bool,
    time: f32,
}

impl App for GalleryApp {
    type Message = Message;

    fn update(&mut self, message: Self::Message, _context: &mut UpdateContext) {
        match message {
            Message::TextChanged(text, cursor) => {
                self.input_text = text;
                self.cursor_index = cursor;
            }
            Message::ButtonClicked(label) => {
                println!("Button clicked: {}", label);
            }
            Message::ToggleChanged(val) => {
                self.toggle_on = val;
            }
            Message::SliderChanged(val) => {
                self.slider_value = val;
            }
            Message::CheckboxChanged(val) => {
                self.checkbox_checked = val;
            }
            Message::ThemeModeChanged(mode) => {
                self.theme_mode = mode;
            }
            Message::TogglePulse(val) => {
                self.is_pulsing = val;
            }
            Message::ToggleFloat(val) => {
                self.is_floating = val;
            }
        }
    }

    fn tick(&mut self, dt: f32) {
        self.time += dt;
    }

    fn view(&self) -> Box<dyn lever_core::widget::Widget<Self::Message>> {
        let theme = lever_core::theme::Theme::for_mode(self.theme_mode);

        // Animation values
        let pulse_scale = if self.is_pulsing {
            1.0 + (self.time * 5.0).sin() * 0.1
        } else {
            1.0
        };

        let float_offset = if self.is_floating {
            (self.time * 3.0).sin() * 10.0
        } else {
            0.0
        };

        let animated_pulse = lever_core::animated::animated_spring(
            "pulse-val",
            pulse_scale,
            lever_core::animation::Spring::SMOOTH,
        );

        let animated_float = lever_core::animated::animated_spring(
            "float-val",
            float_offset,
            lever_core::animation::Spring::SMOOTH,
        );

        let animated_opacity = lever_core::animated::animated_spring(
            "fade-val",
            if self.toggle_on { 1.0 } else { 0.3 },
            lever_core::animation::Spring::SMOOTH,
        );

        Box::new(
            BoxWidget::new(theme.background)
                .with_padding(SideOffsets::all(40.0))
                .with_child(Box::new(
                    Flex::column(vec![
                        // Header row
                        Box::new(
                            Flex::row(vec![
                                Box::new(
                                    Flex::column(vec![
                                        Box::new(Label::new("Lever UI Gallery", 32.0, theme.text)),
                                        Box::new(Label::new(
                                            "A showcase of core widgets.",
                                            16.0,
                                            theme.text_muted,
                                        )),
                                    ])
                                    .with_flex(1),
                                ),
                                Box::new(
                                    ThemeToggle::new("theme-toggle", self.theme_mode)
                                        .on_changed(|mode| Message::ThemeModeChanged(mode)),
                                ),
                            ])
                            .with_gap(20.0),
                        ),
                        Box::new(Spacer::new().with_size(10.0, 30.0)),
                        // Animation Showcase Section
                        Box::new(Label::new("Animation Showcase", 20.0, theme.text)),
                        Box::new(Spacer::new().with_size(10.0, 10.0)),
                        Box::new(
                            Flex::row(vec![
                                // Pulsing Box
                                Box::new(AnimatedScale::new(
                                    animated_pulse,
                                    Box::new(
                                        BoxWidget::new(theme.primary)
                                            .with_radius(8.0)
                                            .with_size(120.0, 80.0)
                                            .with_alignment(Alignment::Center)
                                            .with_child(Box::new(Label::new(
                                                "Pulse",
                                                32.0,
                                                Color::WHITE,
                                            ))),
                                    ),
                                )),
                                // Floating Box
                                Box::new(AnimatedTranslation::new(
                                    Point {
                                        x: 0.0,
                                        y: animated_float,
                                    },
                                    Box::new(
                                        BoxWidget::new(theme.success)
                                            .with_radius(8.0)
                                            .with_size(120.0, 80.0)
                                            .with_alignment(Alignment::Center)
                                            .with_child(Box::new(Label::new(
                                                "Float",
                                                22.0,
                                                Color::WHITE,
                                            ))),
                                    ),
                                )),
                                // Fading Box
                                Box::new(AnimatedOpacity::new(
                                    animated_opacity,
                                    Box::new(
                                        BoxWidget::new(theme.danger)
                                            .with_radius(8.0)
                                            .with_size(120.0, 80.0)
                                            .with_alignment(Alignment::Center)
                                            .with_child(Box::new(Label::new(
                                                "Fade",
                                                14.0,
                                                Color::WHITE,
                                            ))),
                                    ),
                                )),
                                // Controls
                                Box::new(
                                    Flex::column(vec![
                                        Box::new(
                                            Flex::row(vec![
                                                Box::new(Label::new("Pulse", 14.0, theme.text)),
                                                Box::new(
                                                    Toggle::new("pulse-toggle", self.is_pulsing)
                                                        .on_changed(|v| Message::TogglePulse(v)),
                                                ),
                                            ])
                                            .with_gap(10.0),
                                        ),
                                        Box::new(
                                            Flex::row(vec![
                                                Box::new(Label::new("Float", 14.0, theme.text)),
                                                Box::new(
                                                    Toggle::new("float-toggle", self.is_floating)
                                                        .on_changed(|v| Message::ToggleFloat(v)),
                                                ),
                                            ])
                                            .with_gap(10.0),
                                        ),
                                    ])
                                    .with_gap(10.0),
                                ),
                            ])
                            .with_gap(30.0),
                        ),
                        Box::new(Spacer::new().with_size(10.0, 30.0)),
                        // Grid Section
                        Box::new(Label::new("Grid Layout System", 20.0, theme.text)),
                        Box::new(Spacer::new().with_size(10.0, 10.0)),
                        Box::new(
                            Grid::new(
                                vec![GridTrack::Flex(1), GridTrack::Flex(1), GridTrack::Flex(1)],
                                vec![
                                    Box::new(
                                        BoxWidget::new(theme.primary)
                                            .with_radius(8.0)
                                            .with_padding(SideOffsets::all(20.0))
                                            .with_alignment(Alignment::Center)
                                            .with_child(Box::new(Label::new(
                                                "Centered",
                                                14.0,
                                                Color::WHITE,
                                            ))),
                                    ),
                                    Box::new(
                                        BoxWidget::new(theme.success)
                                            .with_radius(8.0)
                                            .with_padding(SideOffsets::all(20.0))
                                            .with_alignment(Alignment::BottomRight)
                                            .with_child(Box::new(Label::new(
                                                "Bottom Right",
                                                14.0,
                                                Color::WHITE,
                                            ))),
                                    ),
                                    Box::new(
                                        BoxWidget::new(theme.danger)
                                            .with_radius(8.0)
                                            .with_padding(SideOffsets::all(20.0))
                                            .on_click(|| {
                                                Message::ButtonClicked("Box Clicked!".into())
                                            })
                                            .with_child(Box::new(Label::new(
                                                "Click Me",
                                                14.0,
                                                Color::WHITE,
                                            ))),
                                    ),
                                ],
                            )
                            .with_gap(15.0),
                        ),
                        Box::new(Spacer::new().with_size(10.0, 30.0)),
                        // Interactive Section
                        Box::new(Label::new("Interactive Widgets", 20.0, theme.text)),
                        Box::new(Spacer::new().with_size(10.0, 10.0)),
                        Box::new(
                            Flex::row(vec![
                                Box::new(
                                    Button::new("btn-primary", "Primary")
                                        .with_variant(ButtonVariant::Primary)
                                        .on_click(|| Message::ButtonClicked("Primary".into())),
                                ),
                                Box::new(
                                    Button::new("btn-secondary", "Secondary")
                                        .with_variant(ButtonVariant::Secondary)
                                        .on_click(|| Message::ButtonClicked("Secondary".into())),
                                ),
                                Box::new(
                                    Button::new("btn-outline", "Outline")
                                        .with_variant(ButtonVariant::Outline)
                                        .on_click(|| Message::ButtonClicked("Outline".into())),
                                ),
                                Box::new(
                                    Button::new("btn-ghost", "Ghost")
                                        .with_variant(ButtonVariant::Ghost)
                                        .on_click(|| Message::ButtonClicked("Ghost".into())),
                                ),
                                Box::new(
                                    Button::new("btn-danger", "Danger")
                                        .with_variant(ButtonVariant::Danger)
                                        .on_click(|| Message::ButtonClicked("Danger".into())),
                                ),
                            ])
                            .with_gap(15.0),
                        ),
                        Box::new(Spacer::new().with_size(10.0, 10.0)),
                        Box::new(
                            Flex::row(vec![
                                Box::new(
                                    Button::new("btn-small", "Small")
                                        .with_size(ButtonSize::Small)
                                        .on_click(|| Message::ButtonClicked("Small".into())),
                                ),
                                Box::new(
                                    Button::new("btn-medium", "Medium")
                                        .with_size(ButtonSize::Medium)
                                        .on_click(|| Message::ButtonClicked("Medium".into())),
                                ),
                                Box::new(
                                    Button::new("btn-large", "Large")
                                        .with_size(ButtonSize::Large)
                                        .on_click(|| Message::ButtonClicked("Large".into())),
                                ),
                                Box::new(
                                    Toggle::new("gallery-toggle", self.toggle_on)
                                        .on_changed(|val| Message::ToggleChanged(val)),
                                ),
                            ])
                            .with_gap(15.0),
                        ),
                    ])
                    .with_gap(20.0),
                )),
        )
    }
}

fn main() {
    let config = AppConfig {
        title: "Lever UI Gallery".to_string(),
        width: 1000,
        height: 850,
        clear_color: Color::rgb(0.05, 0.05, 0.05),
    };

    let app = GalleryApp {
        input_text: "Hello Lever!".to_string(),
        cursor_index: 12,
        toggle_on: true,
        slider_value: 0.5,
        checkbox_checked: false,
        theme_mode: ThemeMode::Dark,
        is_pulsing: true,
        is_floating: true,
        time: 0.0,
    };

    let application = Application::new(config, app);
    application.run();
}
