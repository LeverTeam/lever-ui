use lever_core::app::App;
use lever_core::types::{Color, SideOffsets};
use lever_core::widgets::{
    BoxWidget, Button, Center, Checkbox, Flex, Label, ScrollWidget, Slider, Spacer, TextInput,
    Toggle,
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
}

struct GalleryApp {
    input_text: String,
    cursor_index: usize,
    toggle_on: bool,
    slider_value: f32,
    checkbox_checked: bool,
}

impl App for GalleryApp {
    type Message = Message;

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::TextChanged(new_text, new_cursor) => {
                self.input_text = new_text;
                self.cursor_index = new_cursor;
            }
            Message::ButtonClicked(name) => {
                println!("Button clicked: {}", name);
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
        }
    }

    fn view(&self) -> Box<dyn lever_core::widget::Widget<Self::Message>> {
        Box::new(Center::new(Box::new(
            BoxWidget::new(Color::rgb(0.1, 0.1, 0.1))
                .with_padding(SideOffsets::all(40.0))
                .with_child(Box::new(
                    Flex::column(vec![
                        Box::new(Label::new(
                            "Lever UI Gallery",
                            32.0,
                            Color::rgb(1.0, 1.0, 1.0),
                        )),
                        Box::new(Label::new(
                            "A showcase of core widgets and interaction features.",
                            16.0,
                            Color::rgba(1.0, 1.0, 1.0, 0.5),
                        )),
                        Box::new(Spacer::new().with_size(10.0, 10.0)),
                        Box::new(
                            ScrollWidget::new(Box::new(
                                Flex::column(vec![
                                    // Text Input Section
                                    Box::new(Label::new(
                                        "Text Input",
                                        20.0,
                                        Color::rgb(1.0, 1.0, 1.0),
                                    )),
                                    Box::new(
                                        TextInput::new("gallery-input")
                                            .with_placeholder("Try clicking to position cursor...")
                                            .with_text(&self.input_text)
                                            .with_cursor(self.cursor_index)
                                            .on_input(|text, cursor| {
                                                Message::TextChanged(text, cursor)
                                            }),
                                    ),
                                    Box::new(Spacer::new().with_size(20.0, 20.0)),
                                    // Buttons Section
                                    Box::new(Label::new(
                                        "Buttons (Hover me!)",
                                        20.0,
                                        Color::rgb(1.0, 1.0, 1.0),
                                    )),
                                    Box::new(
                                        Flex::row(vec![
                                            Box::new(
                                                Button::new("Primary")
                                                    .with_color(Color::rgb(0.2, 0.4, 0.8))
                                                    .on_click(|| {
                                                        Message::ButtonClicked("Primary".into())
                                                    }),
                                            ),
                                            Box::new(
                                                Button::new("Success")
                                                    .with_color(Color::rgb(0.2, 0.6, 0.3))
                                                    .on_click(|| {
                                                        Message::ButtonClicked("Success".into())
                                                    }),
                                            ),
                                            Box::new(
                                                Button::new("Danger")
                                                    .with_color(Color::rgb(0.8, 0.2, 0.2))
                                                    .on_click(|| {
                                                        Message::ButtonClicked("Danger".into())
                                                    }),
                                            ),
                                        ])
                                        .with_gap(10.0),
                                    ),
                                    Box::new(Spacer::new().with_size(20.0, 20.0)),
                                    // Interactive Section
                                    Box::new(Label::new(
                                        "Interactive Widgets",
                                        20.0,
                                        Color::rgb(1.0, 1.0, 1.0),
                                    )),
                                    Box::new(
                                        Flex::row(vec![
                                            Box::new(Label::new(
                                                "Toggle:",
                                                16.0,
                                                Color::rgb(1.0, 1.0, 1.0),
                                            )),
                                            Box::new(
                                                Toggle::new("gallery-toggle", self.toggle_on)
                                                    .on_changed(|val| Message::ToggleChanged(val)),
                                            ),
                                            Box::new(Spacer::new().with_size(20.0, 1.0)),
                                            Box::new(
                                                Checkbox::new(
                                                    "gallery-checkbox",
                                                    self.checkbox_checked,
                                                )
                                                .with_label("Enable Feature")
                                                .on_changed(|val| Message::CheckboxChanged(val)),
                                            ),
                                        ])
                                        .with_gap(10.0),
                                    ),
                                    Box::new(Spacer::new().with_size(10.0, 10.0)),
                                    Box::new(
                                        Flex::column(vec![
                                            Box::new(Label::new(
                                                format!("Slider Value: {:.2}", self.slider_value),
                                                16.0,
                                                Color::rgb(1.0, 1.0, 1.0),
                                            )),
                                            Box::new(
                                                Slider::new("gallery-slider", self.slider_value)
                                                    .on_changed(|val| Message::SliderChanged(val)),
                                            ),
                                        ])
                                        .with_gap(5.0),
                                    ),
                                ])
                                .with_gap(15.0),
                            ))
                            .with_flex(1),
                        ),
                    ])
                    .with_gap(10.0),
                )),
        )))
    }
}

fn main() {
    let config = AppConfig {
        title: "Lever UI Gallery".to_string(),
        width: 900,
        height: 700,
        clear_color: Color::rgb(0.05, 0.05, 0.05),
    };

    let app = GalleryApp {
        input_text: "Hello Lever!".to_string(),
        cursor_index: 12,
        toggle_on: true,
        slider_value: 0.5,
        checkbox_checked: false,
    };

    let application = Application::new(config, app);
    application.run();
}
