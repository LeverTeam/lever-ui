# Architecture

## Overview
Lever UI is a modular, reactive UI framework for Rust, built on a message-based update loop inspired by The Elm Architecture. It aims to provide high-performance rendering with a clean, functional API.

## Structure
The project is divided into three primary crates:

### [lever-core](../lever-core)
Contains the platform-agnostic UI logic.
- **Widgets**: The `Widget` trait and standard components like `Button`, `Label`, `Flex`, and `ScrollWidget`.
- **Layout**: A constraint-based layout engine (Flexbox and Grid).
- **Text System**: Text shaping and caching using `cosmic-text`.
- **Theming**: A token-based design system supporting dark/light modes.

### [lever-renderer](../lever-renderer)
The graphics backend.
- **Renderer**: An OpenGL 3.3+ backend using `glow`.
- **Batching**: Efficient geometry batching to reduce draw calls.
- **Atlas**: Glyph atlas management for fast text rendering.
- **Shaders**: Custom shaders for rounded rectangles, gradients, and shadows.

### [lever-windowing](../lever-windowing)
Platform integration.
- **Application**: The main event loop powered by `winit` and `glutin`.
- **Input**: Translation of OS events into the internal `FrameworkEvent` system.

## Data Flow
Lever follows a strict unidirectional data flow:

1. **Event**: An OS event (e.g., mouse click) is received by `lever-windowing`.
2. **Dispatch**: The event is dispatched down the widget tree via `Widget::on_event`.
3. **Message**: Widgets produce messages which are collected and sent to `App::update`.
4. **Update**: `App::update` modifies the application state.
5. **View**: `App::view` is called to rebuild the virtual widget tree.
6. **Layout & Draw**: The framework calculates the layout and calls `Widget::draw` to generate a `DrawList`.
7. **Render**: The `Renderer` consumes the `DrawList` and draws the frame to the GPU.

## Dependencies
- **cosmic-text**: Advanced text shaping and layout.
- **fontdue**: Fast font rasterization.
- **glow**: Cross-platform OpenGL bindings.
- **winit**: Window creation and event handling.
- **glutin**: OpenGL context management.

## Configuration
Lever apps are configured via `AppConfig` in `lever-windowing`:
- `title`: Window title.
- `width` / `height`: Initial window dimensions.
- `transparent`: Enable window transparency.
- `decorations`: Toggle window borders/title bar.
