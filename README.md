# Lever UI

![License](https://img.shields.io/badge/license-MIT-green?style=flat-square)

A cross-platform GPU-accelerated UI framework for Rust with a message-driven architecture. Lever uses Signed Distance Fields (SDF) for high-performance rendering of pixel-perfect rounded shapes, dynamic gradients, and drop shadows.

## Stack

![Rust](https://img.shields.io/badge/Rust-000000?style=flat-square&logo=rust&logoColor=white)
![OpenGL](https://img.shields.io/badge/OpenGL-5586A4?style=flat-square&logo=opengl&logoColor=white)

## Installation

```powershell
cargo build
```

## Usage / Examples

### gallery.rs

A showcase of core widgets including Flex layouts, TextInput, Buttons, Toggles, and Sliders.

```rust
impl App for GalleryApp {
    type Message = Message;

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::TextChanged(new_text, new_cursor) => {
                self.input_text = new_text;
                self.cursor_index = new_cursor;
            }
            Message::ButtonClicked(name) => println!("Clicked: {}", name),
            // ...
        }
    }

    fn view(&self) -> Box<dyn Widget<Self::Message>> {
        Box::new(Center::new(Box::new(
            BoxWidget::new(Color::rgb(0.1, 0.1, 0.1))
                .with_padding(SideOffsets::all(40.0))
                .with_child(Box::new(
                    Flex::column(vec![
                        Box::new(Label::new("Lever UI Gallery", 32.0, Color::rgb(1.0, 1.0, 1.0))),
                        Box::new(TextInput::new("input")
                            .with_text(&self.input_text)
                            .on_input(|t, c| Message::TextChanged(t, c))),
                        Box::new(Flex::row(vec![
                            Box::new(Button::new("Primary").with_color(Color::rgb(0.2, 0.4, 0.8))),
                            Box::new(Toggle::new("toggle", true)),
                        ])),
                        Box::new(Slider::new("slider", 0.5)),
                    ])
                ))
        )))
    }
}
```
