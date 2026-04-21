use crate::draw::DrawList;
use crate::event::{FrameworkEvent, Key};
use crate::layout::{Constraints, LayoutNode, LayoutResult};
use crate::types::{Color, Point, Rect, Size};
use crate::widget::Widget;

pub struct TextInput<M> {
    pub id: String,
    pub text: String,
    pub placeholder: String,
    pub cursor_index: usize,
    pub selection_start: Option<usize>,
    pub on_input: Option<Box<dyn Fn(String, usize) -> M>>,
}

impl<M> TextInput<M> {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            text: String::new(),
            placeholder: String::from("Enter text..."),
            cursor_index: 0,
            selection_start: None,
            on_input: None,
        }
    }

    pub fn with_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = text.into();
        // Clamp cursor if text changed
        if self.cursor_index > self.text.len() {
            self.cursor_index = self.text.len();
        }
        self
    }

    pub fn with_cursor(mut self, index: usize) -> Self {
        self.cursor_index = index.min(self.text.len());
        self
    }

    pub fn with_selection(mut self, start: Option<usize>) -> Self {
        self.selection_start = start.map(|s| s.min(self.text.len()));
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
        let display_text = if self.text.is_empty() && !is_focused {
            &self.placeholder
        } else {
            &self.text
        };
        let color = if self.text.is_empty() && !is_focused {
            Color::rgba(1.0, 1.0, 1.0, 0.3)
        } else {
            theme.text
        };

        let layout = text_system.shape(display_text, 16.0, color);
        let y_offset = (inner_rect.height - layout.height) / 2.0;

        // Draw selection
        if is_focused {
            if let Some(start) = self.selection_start {
                let end = self.cursor_index;
                let (min, max) = if start < end {
                    (start, end)
                } else {
                    (end, start)
                };

                if min != max {
                    // Calculate selection bounds
                    let x_start = if min == 0 {
                        0.0
                    } else {
                        text_system.shape(&self.text[..min], 16.0, color).width
                    };
                    let x_end = text_system.shape(&self.text[..max], 16.0, color).width;

                    draw_list.rounded_rect(
                        Rect {
                            x: inner_rect.x + x_start,
                            y: inner_rect.y + 4.0,
                            width: x_end - x_start,
                            height: inner_rect.height - 8.0,
                        },
                        Color::rgba(theme.primary.r, theme.primary.g, theme.primary.b, 0.3),
                        1.0,
                    );
                }
            }
        }

        draw_list.text(
            Point {
                x: inner_rect.x,
                y: inner_rect.y + y_offset,
            },
            layout.glyphs,
        );

        // Cursor
        if is_focused {
            let cursor_x = if self.cursor_index == 0 {
                0.0
            } else {
                text_system
                    .shape(&self.text[..self.cursor_index], 16.0, color)
                    .width
            };

            draw_list.rounded_rect(
                Rect {
                    x: inner_rect.x + cursor_x,
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
        event: &FrameworkEvent,
        rect: Rect,
        _text_system: &mut crate::text::TextSystem,
        _theme: &crate::theme::Theme,
        focused_id: &mut Option<String>,
    ) -> Vec<M> {
        let mut messages = Vec::new();
        match event {
            FrameworkEvent::PointerDown { position, .. } => {
                if rect.contains(*position) {
                    *focused_id = Some(self.id.clone());
                    // TODO: Calculate cursor index from click position
                } else if focused_id.as_deref() == Some(&self.id) {
                    *focused_id = None;
                }
            }
            FrameworkEvent::TextInput { text } => {
                if focused_id.as_deref() == Some(&self.id) {
                    let mut new_text = self.text.clone();
                    let mut new_cursor = self.cursor_index;

                    // Handle selection replacement
                    if let Some(start) = self.selection_start {
                        let (min, max) = if start < new_cursor {
                            (start, new_cursor)
                        } else {
                            (new_cursor, start)
                        };
                        new_text.replace_range(min..max, "");
                        new_cursor = min;
                    }

                    for c in text.chars() {
                        if !c.is_control() {
                            new_text.insert(new_cursor, c);
                            new_cursor += c.len_utf8();
                        }
                    }

                    if let Some(on_input) = &self.on_input {
                        messages.push(on_input(new_text, new_cursor));
                    }
                }
            }
            FrameworkEvent::KeyDown { key, modifiers } => {
                if focused_id.as_deref() == Some(&self.id) {
                    let mut new_text = self.text.clone();
                    let mut new_cursor = self.cursor_index;
                    let mut new_selection = self.selection_start;

                    match key {
                        Key::Left => {
                            if !modifiers.shift {
                                if let Some(start) = new_selection {
                                    new_cursor = if start < new_cursor {
                                        start
                                    } else {
                                        new_cursor
                                    };
                                    new_selection = None;
                                } else if new_cursor > 0 {
                                    // Move by char
                                    new_cursor = self.text[..new_cursor]
                                        .chars()
                                        .next_back()
                                        .map(|c| new_cursor - c.len_utf8())
                                        .unwrap_or(0);
                                }
                            } else {
                                if new_selection.is_none() {
                                    new_selection = Some(self.cursor_index);
                                }
                                if new_cursor > 0 {
                                    new_cursor = self.text[..new_cursor]
                                        .chars()
                                        .next_back()
                                        .map(|c| new_cursor - c.len_utf8())
                                        .unwrap_or(0);
                                }
                            }
                        }
                        Key::Right => {
                            if !modifiers.shift {
                                if let Some(start) = new_selection {
                                    new_cursor = if start > new_cursor {
                                        start
                                    } else {
                                        new_cursor
                                    };
                                    new_selection = None;
                                } else if new_cursor < self.text.len() {
                                    new_cursor = self.text[new_cursor..]
                                        .chars()
                                        .next()
                                        .map(|c| new_cursor + c.len_utf8())
                                        .unwrap_or(self.text.len());
                                }
                            } else {
                                if new_selection.is_none() {
                                    new_selection = Some(self.cursor_index);
                                }
                                if new_cursor < self.text.len() {
                                    new_cursor = self.text[new_cursor..]
                                        .chars()
                                        .next()
                                        .map(|c| new_cursor + c.len_utf8())
                                        .unwrap_or(self.text.len());
                                }
                            }
                        }
                        Key::Backspace => {
                            if let Some(start) = new_selection {
                                let (min, max) = if start < new_cursor {
                                    (start, new_cursor)
                                } else {
                                    (new_cursor, start)
                                };
                                new_text.replace_range(min..max, "");
                                new_cursor = min;
                                new_selection = None;
                            } else if new_cursor > 0 {
                                let prev_char_len = self.text[..new_cursor]
                                    .chars()
                                    .next_back()
                                    .map(|c| c.len_utf8())
                                    .unwrap_or(0);
                                new_text.remove(new_cursor - prev_char_len);
                                new_cursor -= prev_char_len;
                            }
                        }
                        Key::Delete => {
                            if let Some(start) = new_selection {
                                let (min, max) = if start < new_cursor {
                                    (start, new_cursor)
                                } else {
                                    (new_cursor, start)
                                };
                                new_text.replace_range(min..max, "");
                                new_cursor = min;
                                new_selection = None;
                            } else if new_cursor < self.text.len() {
                                new_text.remove(new_cursor);
                            }
                        }
                        Key::Home => {
                            if modifiers.shift && new_selection.is_none() {
                                new_selection = Some(new_cursor);
                            } else if !modifiers.shift {
                                new_selection = None;
                            }
                            new_cursor = 0;
                        }
                        Key::End => {
                            if modifiers.shift && new_selection.is_none() {
                                new_selection = Some(new_cursor);
                            } else if !modifiers.shift {
                                new_selection = None;
                            }
                            new_cursor = self.text.len();
                        }
                        _ => {}
                    }

                    if new_text != self.text
                        || new_cursor != self.cursor_index
                        || new_selection != self.selection_start
                    {
                        if let Some(on_input) = &self.on_input {
                            // We need a way to pass selection back too.
                            // For now I'll just pass text and cursor.
                            messages.push(on_input(new_text, new_cursor));
                        }
                    }
                }
            }
            _ => {}
        }
        messages
    }
}
