use crate::draw::DrawList;
use crate::layout::{Constraints, LayoutNode, LayoutResult};
use crate::types::{Color, Rect, Size};
use crate::widget::Widget;

pub struct TextInput<M> {
    pub id: String,
    pub text: String,
    pub placeholder: String,
    pub on_input: Option<Box<dyn Fn(String) -> M>>,
}

impl<M> TextInput<M> {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            text: String::new(),
            placeholder: String::from("Enter text..."),
            on_input: None,
        }
    }

    pub fn with_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = text.into();
        self
    }

    pub fn on_input<F>(mut self, f: F) -> Self
    where
        F: Fn(String) -> M + 'static,
    {
        self.on_input = Some(Box::new(f));
        self
    }
}

impl<M: 'static> Widget<M> for TextInput<M> {
    fn id(&self) -> Option<&str> {
        Some(&self.id)
    }

    fn layout(
        &self,
        constraints: Constraints,
        _children: &[LayoutNode],
        _text_system: &mut crate::text::TextSystem,
        _theme: &crate::theme::Theme,
    ) -> LayoutResult {
        let size = constraints.clamp_size(Size {
            width: 250.0,
            height: 40.0,
        });
        LayoutResult { size }
    }

    fn draw(
        &self,
        rect: Rect,
        draw_list: &mut DrawList,
        text_system: &mut crate::text::TextSystem,
        theme: &crate::theme::Theme,
        focused_id: Option<&str>,
    ) {
        let is_focused = focused_id == Some(&self.id);

        // Background
        draw_list.rounded_rect(rect, theme.surface, theme.radius_md);

        // Focus indicator
        if is_focused {
            draw_list.stroke_rect(rect, theme.primary, theme.radius_md, 2.0);
        }

        let inner_rect = rect.inset(10.0, 5.0);
        let display_text = if self.text.is_empty() {
            &self.placeholder
        } else {
            &self.text
        };
        let color = if self.text.is_empty() {
            Color::rgba(1.0, 1.0, 1.0, 0.3)
        } else {
            theme.text
        };

        let layout = text_system.shape(display_text, 16.0, color);
        let y_offset = (inner_rect.height - layout.height) / 2.0;
        draw_list.text(
            crate::types::Point {
                x: inner_rect.x,
                y: inner_rect.y + y_offset,
            },
            layout.glyphs,
        );

        // Cursor
        if is_focused {
            let cursor_x = inner_rect.x
                + if self.text.is_empty() {
                    0.0
                } else {
                    layout.width
                };
            draw_list.rounded_rect(
                Rect {
                    x: cursor_x,
                    y: inner_rect.y + 4.0,
                    width: 2.0,
                    height: inner_rect.height - 8.0,
                },
                theme.primary,
                1.0,
            );
        }
    }

    fn on_event(
        &mut self,
        event: &crate::event::FrameworkEvent,
        rect: Rect,
        _text_system: &mut crate::text::TextSystem,
        _theme: &crate::theme::Theme,
        focused_id: &mut Option<String>,
    ) -> Vec<M> {
        let mut messages = Vec::new();
        match event {
            crate::event::FrameworkEvent::PointerDown { position, .. } => {
                if rect.contains(*position) {
                    *focused_id = Some(self.id.clone());
                } else if focused_id.as_deref() == Some(&self.id) {
                    *focused_id = None;
                }
            }
            crate::event::FrameworkEvent::TextInput { text } => {
                if focused_id.as_deref() == Some(&self.id) {
                    let mut new_text = self.text.clone();
                    for c in text.chars() {
                        if !c.is_control() {
                            new_text.push(c);
                        }
                    }
                    if new_text != self.text {
                        if let Some(on_input) = &self.on_input {
                            messages.push(on_input(new_text));
                        }
                    }
                }
            }
            crate::event::FrameworkEvent::KeyDown { key, .. } => {
                if focused_id.as_deref() == Some(&self.id) {
                    if *key == crate::event::Key::Backspace {
                        let mut new_text = self.text.clone();
                        new_text.pop();
                        if let Some(on_input) = &self.on_input {
                            messages.push(on_input(new_text));
                        }
                    }
                }
            }
            _ => {}
        }
        messages
    }
}
