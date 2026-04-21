Lever UI

A cross-platform UI framework for Rust with GPU-accelerated rendering and a stateful, message-driven architecture. It uses Signed Distance Fields (SDF) for high-quality rounded shapes, gradients, and shadows.

## Installation

```powershell
# Build the project
cargo build

# Run examples
cargo run --example hello
cargo run --example animation_demo
```

## Usage / Examples

### hello.rs

Basic application setup with a single centered label.

```rust
impl App for HelloApp {
    type Message = Message;
    fn update(&mut self, _message: Self::Message) {}
    fn view(&self) -> Box<dyn Widget<Self::Message>> {
        Box::new(Center::new(Box::new(
            BoxWidget::new(Color::rgb(0.2, 0.4, 0.2))
                .with_child(Box::new(Label::new("Hello Lever!", 32.0, Color::rgb(1.0, 1.0, 1.0)))),
        )))
    }
}
```

### animation_demo.rs

Smooth color transitions using the built-in animation system and easing functions.

```rust
impl App for AnimationDemo {
    fn tick(&mut self, dt: f32) {
        self.hover_anim.update(dt);
    }
    fn view(&self) -> Box<dyn Widget<Self::Message>> {
        let button_color = self.hover_anim.color(start_color, end_color);
        Box::new(Button::new("Animated Button").with_color(button_color))
    }
}
```
