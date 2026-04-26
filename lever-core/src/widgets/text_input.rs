use crate::draw::DrawList;
use crate::event::{FrameworkEvent, PointerButton};
use crate::layout::{Constraints, LayoutNode, LayoutResult};
use crate::types::{Point, Rect, Size};
use crate::widget::Widget;

pub struct TextInput<M> {
    pub id: String,
    pub text: String,
    pub placeholder: String,
    pub cursor_index: usize,
    pub on_input: Option<Box<dyn Fn(String, usize) -> M>>,
}

impl<M> TextInput<M> {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            text: String::new(),
            placeholder: String::new(),
            cursor_index: 0,
            on_input: None,
        }
    }

    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = text.into();
        self.cursor_index = self.text.len();
        self
    }

    pub fn with_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn with_cursor(mut self, index: usize) -> Self {
        self.cursor_index = index.min(self.text.len());
        self
    }

    pub fn on_input<F>(mut self, f: F) -> Self
    where
        F: Fn(String, usize) -> M + 'static,
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
            width: constraints.max_width.min(300.0),
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
        _pointer_pos: Option<crate::types::Point>,
    ) {
        let is_focused = focused_id == Some(&self.id);

        // Background
        draw_list.rounded_rect(rect, theme.surface_variant, theme.radius_md);

        // Border
        if is_focused {
            draw_list.stroke_rect(rect, theme.primary, theme.radius_md, 2.0);
        } else {
            draw_list.stroke_rect(rect, theme.border, theme.radius_md, 1.0);
        }

        let inner_rect = rect.inset(12.0, 0.0);

        if self.text.is_empty() {
            let layout = text_system.shape(&self.placeholder, 14.0, theme.text_muted);
            draw_list.text(
                Point {
                    x: inner_rect.x,
                    y: inner_rect.y + (inner_rect.height - layout.height) / 2.0,
                },
                layout.glyphs,
            );
        } else {
            let layout = text_system.shape(&self.text, 14.0, theme.text);
            draw_list.text(
                Point {
                    x: inner_rect.x,
                    y: inner_rect.y + (inner_rect.height - layout.height) / 2.0,
                },
                layout.glyphs.clone(),
            );

            if is_focused {
                // Draw cursor
                let cursor_pos = layout.cursor_to_pos(self.cursor_index);
                draw_list.rounded_rect(
                    Rect {
                        x: inner_rect.x + cursor_pos,
                        y: inner_rect.y + (inner_rect.height - 18.0) / 2.0,
                        width: 2.0,
                        height: 18.0,
                    },
                    theme.primary,
                    1.0,
                );
            }
        }
    }

    fn on_event(
        &mut self,
        event: &FrameworkEvent,
        rect: Rect,
        text_system: &mut crate::text::TextSystem,
        _theme: &crate::theme::Theme,
        focused_id: &mut Option<String>,
    ) -> Vec<M> {
        let mut messages = Vec::new();
        match event {
            FrameworkEvent::PointerDown { position, button } => {
                if *button == PointerButton::Primary && rect.contains(*position) {
                    *focused_id = Some(self.id.clone());

                    let inner_rect = rect.inset(12.0, 0.0);
                    self.cursor_index =
                        text_system.hit_test(&self.text, 14.0, position.x - inner_rect.x);

                    if let Some(on_input) = &self.on_input {
                        messages.push(on_input(self.text.clone(), self.cursor_index));
                    }
                }
            }
            FrameworkEvent::TextInput { text } => {
                if focused_id.as_deref() == Some(&self.id) {
                    self.text.insert_str(self.cursor_index, text);
                    self.cursor_index += text.len();
                    if let Some(on_input) = &self.on_input {
                        messages.push(on_input(self.text.clone(), self.cursor_index));
                    }
                }
            }
            FrameworkEvent::KeyDown { key, .. } => {
                if focused_id.as_deref() == Some(&self.id) {
                    match key {
                        crate::event::Key::Backspace => {
                            if self.cursor_index > 0 {
                                self.cursor_index -= 1;
                                self.text.remove(self.cursor_index);
                            }
                        }
                        crate::event::Key::Delete => {
                            if self.cursor_index < self.text.len() {
                                self.text.remove(self.cursor_index);
                            }
                        }
                        crate::event::Key::Left => {
                            if self.cursor_index > 0 {
                                self.cursor_index -= 1;
                            }
                        }
                        crate::event::Key::Right => {
                            if self.cursor_index < self.text.len() {
                                self.cursor_index += 1;
                            }
                        }
                        _ => {}
                    }
                    if let Some(on_input) = &self.on_input {
                        messages.push(on_input(self.text.clone(), self.cursor_index));
                    }
                }
            }
            _ => {}
        }
        messages
    }
}

