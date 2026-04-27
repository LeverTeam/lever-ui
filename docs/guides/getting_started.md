# Getting Started

This guide will help you build your first application with Lever UI.

## Prerequisites
- **Rust**: Latest stable version installed via `rustup`.
- **OpenGL 3.3+**: Ensure your graphics drivers are up to date.
- **Dependencies**:
  - Windows: No extra requirements.
  - Linux: `libwayland-dev`, `libx11-dev`, `libxkbcommon-dev`.

## Step 1: Initialize Project
Create a new Rust project and add the Lever crates:

```bash
cargo new my-lever-app
cd my-lever-app
```

Add these to your `Cargo.toml`:
```toml
[dependencies]
lever-core = { path = "../lever-ui/lever-core" }
lever-windowing = { path = "../lever-ui/lever-windowing" }
```

## Step 2: Create Your App
Define your application state and messages in `src/main.rs`:

```rust
use lever_core::app::{App, Context, UpdateContext};
use lever_core::widget::Widget;
use lever_core::widgets::{Label, Button, Flex, Center};
use lever_windowing::application::{Application, AppConfig};

struct CounterApp {
    count: i32,
}

#[derive(Clone, Debug)]
enum Message {
    Increment,
    Decrement,
}

impl App for CounterApp {
    type Message = Message;

    fn update(&mut self, message: Message, _ctx: &mut UpdateContext) {
        match message {
            Message::Increment => self.count += 1,
            Message::Decrement => self.count -= 1,
        }
    }

    fn view(&self) -> Box<dyn Widget<Message>> {
        Center::new(
            Flex::column(vec![
                Label::new(format!("Count: {}", self.count)).with_size(32.0).into(),
                Flex::row(vec![
                    Button::new("Decrement").on_press(Message::Decrement).into(),
                    Button::new("Increment").on_press(Message::Increment).into(),
                ]).with_gap(10.0).into(),
            ])
            .with_gap(20.0)
            .into()
        ).into()
    }
}

fn main() {
    let app = CounterApp { count: 0 };
    let config = AppConfig {
        title: "Counter App".to_string(),
        ..Default::default()
    };

    Application::new(app, config).run();
}
```

## Step 3: Run the App
Execute your application with cargo:

```bash
cargo run
```

Expected output:
A window with a "Count: 0" label and two buttons that modify the count.

## Core Concepts

### The Message Pattern
Widgets in Lever don't have callbacks that mutate state directly. Instead, they emit **Messages**. Your `App::update` function is the only place where state mutation happens, ensuring that the UI state is always predictable and easy to debug.

### The View Function
The `view()` function is called whenever the application state changes. It returns a fresh tree of widgets. Lever's layout and rendering engines are optimized to handle this "virtual tree" approach efficiently.

### Using `.into()`
Most standard widgets provide a `.into()` method (via `Box<dyn Widget<M>>`) to make it easier to compose them within containers like `Flex` or `Center`.
