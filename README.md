# Lever UI

![Version](https://img.shields.io/badge/version-0.1.0-blue?style=flat-square)
![License](https://img.shields.io/badge/license-MIT-green?style=flat-square)

Lever is a cross-platform, GPU-accelerated UI framework for Rust featuring a message-driven architecture. It utilizes Signed Distance Fields (SDF) to render pixel-perfect rounded shapes, dynamic gradients, and smooth drop shadows with high performance.

## Stack

![Rust](https://img.shields.io/badge/Rust-000000?style=flat-square&logo=rust&logoColor=white)
![OpenGL](https://img.shields.io/badge/OpenGL-5586A4?style=flat-square&logo=opengl&logoColor=white)

## Installation

```powershell
cargo build
```

## Usage / Examples

### Core Widget Implementation

Lever uses a declarative `view` function to construct UIs from composable widgets.

```rust
impl App for MyApp {
    type Message = Message;

    fn update(&mut self, message: Message, context: &mut UpdateContext) {
        match message {
            Message::TogglePulse(val) => self.is_pulsing = val,
            Message::SliderChanged(val) => self.slider_value = val,
        }
    }

    fn view(&self) -> Box<dyn Widget<Self::Message>> {
        Box::new(Flex::column(vec![
            Box::new(Label::new("Lever UI").with_size(32.0)),
            Box::new(
                Flex::row(vec![
                    Box::new(Button::new("btn", "Action").with_variant(ButtonVariant::Primary)),
                    Box::new(Toggle::new("toggle", self.is_pulsing).on_changed(Message::TogglePulse)),
                ])
                .with_gap(12.0),
            ),
            Box::new(ProgressBar::new("progress", self.slider_value)),
            Box::new(
                Wrap::new()
                    .with_spacing(8.0)
                    .with_children(vec![
                        Box::new(Label::new("Tag 1")),
                        Box::new(Label::new("Tag 2")),
                    ]),
            ),
        ]))
    }
}
```

### Running the Gallery

The modular gallery demonstrates all available components including Animations, Progress Indicators, and Advanced Layouts (Stack/Wrap).

```powershell
cargo run --example gallery
```
