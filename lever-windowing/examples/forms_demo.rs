use lever_core::app::{App, UpdateContext};
use lever_core::theme::{Theme, ThemeMode};
use lever_core::types::{Color, SideOffsets};
use lever_core::widget::Widget;
use lever_core::widgets::{
    BoxWidget, Button, Center, Checkbox, Flex, Label, RadioButton, Slider, Spacer,
};
use lever_windowing::application::Application;
use lever_windowing::AppConfig;

struct FormsDemoApp {
    theme_mode: ThemeMode,
    checkbox_1: bool,
    checkbox_2: bool,
    radio_selection: u32,
    slider_val: f32,
    volume: f32,
}

#[derive(Clone, Debug)]
enum Message {
    ToggleTheme,
    Checkbox1Toggled(bool),
    Checkbox2Toggled(bool),
    RadioSelected(u32),
    SliderChanged(f32),
    VolumeChanged(f32),
}

impl App for FormsDemoApp {
    type Message = Message;

    fn update(&mut self, message: Message, _ctx: &mut UpdateContext) {
        match message {
            Message::ToggleTheme => {
                self.theme_mode = match self.theme_mode {
                    ThemeMode::Dark => ThemeMode::Light,
                    ThemeMode::Light => ThemeMode::Dark,
                };
            }
            Message::Checkbox1Toggled(val) => self.checkbox_1 = val,
            Message::Checkbox2Toggled(val) => self.checkbox_2 = val,
            Message::RadioSelected(idx) => self.radio_selection = idx,
            Message::SliderChanged(val) => self.slider_val = val,
            Message::VolumeChanged(val) => self.volume = val,
        }
    }

    fn view(&self) -> Box<dyn Widget<Message>> {
        let theme = Theme::for_mode(self.theme_mode);

        Box::new(
            BoxWidget::new(theme.background)
                .with_padding(SideOffsets::all(40.0))
                .with_child(Box::new(Flex::column(vec![
                    Box::new(Flex::row(vec![
                        Box::new(
                            Flex::column(vec![
                                Box::new(Label::new("Form Components", 32.0, theme.text)),
                                Box::new(Label::new(
                                    "Animated Interactive Widgets",
                                    16.0,
                                    theme.text_muted,
                                )),
                            ])
                            .with_flex(1),
                        ),
                        Box::new(
                            Button::new("Toggle Theme")
                                .with_id("theme_btn")
                                .on_click(|| Message::ToggleTheme),
                        ),
                    ])),
                    Box::new(Spacer::new().with_size(10.0, 40.0)),
                    // Checkboxes
                    Box::new(Label::new("Checkboxes", 18.0, theme.text)),
                    Box::new(Spacer::new().with_size(10.0, 10.0)),
                    Box::new(Flex::row(vec![
                        Box::new(
                            Checkbox::new("check1", self.checkbox_1)
                                .with_label("Enable Notifications")
                                .on_changed(Message::Checkbox1Toggled),
                        ),
                        Box::new(Spacer::new().with_size(20.0, 10.0)),
                        Box::new(
                            Checkbox::new("check2", self.checkbox_2)
                                .with_label("Auto Update")
                                .on_changed(Message::Checkbox2Toggled),
                        ),
                    ])),
                    Box::new(Spacer::new().with_size(10.0, 30.0)),
                    // Radio Buttons
                    Box::new(Label::new("Radio Buttons", 18.0, theme.text)),
                    Box::new(Spacer::new().with_size(10.0, 10.0)),
                    Box::new(Flex::column(vec![
                        Box::new(
                            RadioButton::new("radio1", self.radio_selection == 0)
                                .with_label("Standard Delivery")
                                .on_selected(|| Message::RadioSelected(0)),
                        ),
                        Box::new(Spacer::new().with_size(10.0, 8.0)),
                        Box::new(
                            RadioButton::new("radio2", self.radio_selection == 1)
                                .with_label("Express Delivery")
                                .on_selected(|| Message::RadioSelected(1)),
                        ),
                        Box::new(Spacer::new().with_size(10.0, 8.0)),
                        Box::new(
                            RadioButton::new("radio3", self.radio_selection == 2)
                                .with_label("Overnight Shipping")
                                .on_selected(|| Message::RadioSelected(2)),
                        ),
                    ])),
                    Box::new(Spacer::new().with_size(10.0, 30.0)),
                    // Sliders
                    Box::new(Label::new("Sliders", 18.0, theme.text)),
                    Box::new(Spacer::new().with_size(10.0, 10.0)),
                    Box::new(Flex::column(vec![
                        Box::new(Label::new(
                            format!("Opacity: {}%", (self.slider_val * 100.0) as i32),
                            14.0,
                            theme.text_muted,
                        )),
                        Box::new(
                            Slider::new("slider1", self.slider_val)
                                .on_changed(Message::SliderChanged),
                        ),
                        Box::new(Spacer::new().with_size(10.0, 15.0)),
                        Box::new(Label::new(
                            format!("Volume: {}%", (self.volume * 100.0) as i32),
                            14.0,
                            theme.text_muted,
                        )),
                        Box::new(
                            Slider::new("volume", self.volume).on_changed(Message::VolumeChanged),
                        ),
                    ])),
                    Box::new(Spacer::new().with_size(10.0, 40.0)),
                    // Live Preview Area
                    Box::new(
                        BoxWidget::new(theme.surface)
                            .with_radius(theme.radius_md)
                            .with_padding(SideOffsets::all(20.0))
                            .with_child(Box::new(Center::new(Box::new(Label::new(
                                "Live Preview Card",
                                20.0,
                                Color::rgba(
                                    theme.text.r,
                                    theme.text.g,
                                    theme.text.b,
                                    self.slider_val.max(0.1),
                                ),
                            ))))),
                    ),
                ]))),
        )
    }
}

fn main() {
    let app = FormsDemoApp {
        theme_mode: ThemeMode::Dark,
        checkbox_1: true,
        checkbox_2: false,
        radio_selection: 0,
        slider_val: 0.8,
        volume: 0.5,
    };

    let config = AppConfig {
        title: "Lever Forms Demo".to_string(),
        width: 800,
        height: 800,
        ..Default::default()
    };

    Application::new(config, app).run();
}
