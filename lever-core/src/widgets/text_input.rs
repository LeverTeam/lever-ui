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
    pub selection_start: Option<usize>,
    pub scroll_offset: f32,
    pub is_password: bool,
    pub leading_icon: Option<crate::types::TextureId>,
    pub trailing_icon: Option<crate::types::TextureId>,
    pub error_text: Option<String>,
    pub blink_timer: f32,
    pub flex: u32,
    pub on_input: Option<Box<dyn Fn(String, usize) -> M>>,
}

impl<M> TextInput<M> {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            text: String::new(),
            placeholder: String::new(),
            cursor_index: 0,
            selection_start: None,
            scroll_offset: 0.0,
            is_password: false,
            leading_icon: None,
            trailing_icon: None,
            error_text: None,
            blink_timer: 0.0,
            flex: 0,
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

    pub fn with_password(mut self, is_password: bool) -> Self {
        self.is_password = is_password;
        self
    }

    pub fn with_leading_icon(mut self, icon: crate::types::TextureId) -> Self {
        self.leading_icon = Some(icon);
        self
    }

    pub fn with_trailing_icon(mut self, icon: crate::types::TextureId) -> Self {
        self.trailing_icon = Some(icon);
        self
    }

    pub fn with_error(mut self, error: impl Into<String>) -> Self {
        self.error_text = Some(error.into());
        self
    }

    pub fn with_flex(mut self, flex: u32) -> Self {
        self.flex = flex;
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
        theme: &crate::theme::Theme,
    ) -> LayoutResult {
        let height = theme.font_size_md + theme.padding_md * 1.5;
        let size = constraints.clamp_size(Size {
            width: constraints.max_width.min(300.0),
            height,
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

        let bg_color = if self.error_text.is_some() {
            theme.danger.with_alpha(0.05)
        } else {
            theme.surface_variant
        };
        draw_list.rounded_rect(rect, bg_color, theme.radius_md);

        let border_color = if self.error_text.is_some() {
            theme.danger
        } else if is_focused {
            theme.primary
        } else {
            theme.border
        };
        let border_thickness = if is_focused { 2.0 } else { 1.0 };
        draw_list.stroke_rect(rect, border_color, theme.radius_md, border_thickness);

        let mut inner_rect = rect.inset(12.0, 0.0);
        if let Some(icon) = self.leading_icon {
            let icon_size = 16.0;
            let icon_rect = Rect {
                x: inner_rect.x,
                y: inner_rect.y + (inner_rect.height - icon_size) / 2.0,
                width: icon_size,
                height: icon_size,
            };
            draw_list.textured_rect(icon_rect, icon, theme.text_muted, [0.0, 0.0, 1.0, 1.0]);
            inner_rect.x += icon_size + 8.0;
            inner_rect.width -= icon_size + 8.0;
        }
        if let Some(icon) = self.trailing_icon {
            let icon_size = 16.0;
            let icon_rect = Rect {
                x: rect.x + rect.width - 12.0 - icon_size,
                y: inner_rect.y + (inner_rect.height - icon_size) / 2.0,
                width: icon_size,
                height: icon_size,
            };
            draw_list.textured_rect(icon_rect, icon, theme.text_muted, [0.0, 0.0, 1.0, 1.0]);
            inner_rect.width -= icon_size + 8.0;
        }

        let display_text = if self.is_password {
            "•".repeat(self.text.chars().count())
        } else {
            self.text.clone()
        };

        let mapped_cursor_index = if self.is_password && !self.text.is_empty() {
            self.text[..self.cursor_index].chars().count() * "•".len()
        } else {
            self.cursor_index
        };

        let mapped_selection_start = self.selection_start.map(|start| {
            if self.is_password && !self.text.is_empty() {
                self.text[..start.min(self.text.len())].chars().count() * "•".len()
            } else {
                start
            }
        });

        let layout = text_system.shape(
            if display_text.is_empty() {
                &self.placeholder
            } else {
                &display_text
            },
            14.0,
            if display_text.is_empty() {
                theme.text_muted
            } else {
                theme.text
            },
            None,
            crate::types::TextAlign::Left,
        );

        let mut scroll_offset = self.scroll_offset;
        if is_focused {
            let cursor_pos = layout.cursor_to_pos(mapped_cursor_index);
            let viewport_width = inner_rect.width;

            if cursor_pos - scroll_offset < 0.0 {
                scroll_offset = cursor_pos;
            } else if cursor_pos - scroll_offset > viewport_width - 4.0 {
                scroll_offset = cursor_pos - viewport_width + 4.0;
            }
        }

        let max_scroll = (layout.width - inner_rect.width).max(0.0);
        scroll_offset = scroll_offset.clamp(0.0, max_scroll);

        draw_list.clip_push(inner_rect);

        let text_pos = Point {
            x: inner_rect.x - scroll_offset,
            y: inner_rect.y + (inner_rect.height - layout.height) / 2.0,
        };

        if let (Some(start), true) = (mapped_selection_start, is_focused) {
            let p1 = layout.cursor_to_pos(start);
            let p2 = layout.cursor_to_pos(mapped_cursor_index);
            let x1 = p1.min(p2);
            let x2 = p1.max(p2);

            if x1 != x2 {
                draw_list.rounded_rect(
                    Rect {
                        x: text_pos.x + x1,
                        y: inner_rect.y + 6.0,
                        width: x2 - x1,
                        height: inner_rect.height - 12.0,
                    },
                    theme.primary.with_alpha(0.3),
                    2.0,
                );
            }
        }

        draw_list.text(text_pos, layout.glyphs.clone());

        if is_focused && self.blink_timer.fract() < 0.5 {
            let cursor_pos = layout.cursor_to_pos(mapped_cursor_index);
            let cursor_height = (layout.height * 0.8).max(16.0);
            draw_list.rounded_rect(
                Rect {
                    x: text_pos.x + cursor_pos - 1.0,
                    y: inner_rect.y + (inner_rect.height - cursor_height) / 2.0,
                    width: 2.0,
                    height: cursor_height,
                },
                theme.primary,
                1.0,
            );
        }

        draw_list.clip_pop();

        if let Some(error) = &self.error_text {
            let error_layout = text_system.shape(
                error,
                11.0,
                theme.danger,
                Some(rect.width),
                crate::types::TextAlign::Left,
            );
            draw_list.text(
                Point {
                    x: rect.x + 4.0,
                    y: rect.y + rect.height + 4.0,
                },
                error_layout.glyphs,
            );
        }
    }

    fn on_event(
        &mut self,
        event: &FrameworkEvent,
        rect: Rect,
        text_system: &mut crate::text::TextSystem,
        _theme: &crate::theme::Theme,
        focused_id: &mut Option<String>,
        consumed: &mut bool,
    ) -> Vec<M> {
        let mut messages = Vec::new();
        let is_focused = focused_id.as_deref() == Some(&self.id);

        match event {
            FrameworkEvent::Animate { dt } => {
                if is_focused {
                    self.blink_timer += dt;
                } else {
                    self.blink_timer = 0.0;
                }
            }
            FrameworkEvent::PointerDown { position, button } => {
                if *button == PointerButton::Primary && rect.contains(*position) {
                    *consumed = true;
                    *focused_id = Some(self.id.clone());

                    let mut inner_rect = rect.inset(12.0, 0.0);
                    if self.leading_icon.is_some() {
                        inner_rect.x += 16.0 + 8.0;
                    }

                    let display_text = if self.is_password {
                        "•".repeat(self.text.chars().count())
                    } else {
                        self.text.clone()
                    };

                    let hit_index = text_system.hit_test(
                        &display_text,
                        14.0,
                        position.x - inner_rect.x + self.scroll_offset,
                    );

                    if self.is_password && !self.text.is_empty() {
                        let char_idx = hit_index / "•".len();
                        self.cursor_index = self
                            .text
                            .char_indices()
                            .nth(char_idx)
                            .map(|(idx, _)| idx)
                            .unwrap_or(self.text.len());
                    } else {
                        self.cursor_index = hit_index;
                    }
                    self.selection_start = Some(self.cursor_index);
                    self.blink_timer = 0.0;
                } else if *button == PointerButton::Primary {
                    if is_focused {
                        self.selection_start = None;
                    }
                }
            }
            FrameworkEvent::PointerMove { position } => {
                if is_focused && self.selection_start.is_some() {
                    let mut inner_rect = rect.inset(12.0, 0.0);
                    if self.leading_icon.is_some() {
                        inner_rect.x += 16.0 + 8.0;
                    }

                    let display_text = if self.is_password {
                        "•".repeat(self.text.chars().count())
                    } else {
                        self.text.clone()
                    };

                    let hit_index = text_system.hit_test(
                        &display_text,
                        14.0,
                        position.x - inner_rect.x + self.scroll_offset,
                    );

                    if self.is_password && !self.text.is_empty() {
                        let char_idx = hit_index / "•".len();
                        self.cursor_index = self
                            .text
                            .char_indices()
                            .nth(char_idx)
                            .map(|(idx, _)| idx)
                            .unwrap_or(self.text.len());
                    } else {
                        self.cursor_index = hit_index;
                    }
                    *consumed = true;
                }
            }
            FrameworkEvent::PointerUp { .. } => {
                if is_focused && self.selection_start == Some(self.cursor_index) {
                    self.selection_start = None;
                }
            }
            FrameworkEvent::TextInput { text } => {
                if is_focused {
                    if let Some(start) = self.selection_start {
                        let x1 = start.min(self.cursor_index);
                        let x2 = start.max(self.cursor_index);
                        if x1 != x2 {
                            self.text.replace_range(x1..x2, "");
                            self.cursor_index = x1;
                        }
                        self.selection_start = None;
                    }

                    self.text.insert_str(self.cursor_index, text);
                    self.cursor_index += text.len();
                    self.blink_timer = 0.0;

                    if let Some(on_input) = &self.on_input {
                        messages.push(on_input(self.text.clone(), self.cursor_index));
                    }
                    *consumed = true;
                }
            }
            FrameworkEvent::KeyDown { key, modifiers } => {
                if is_focused {
                    let mut changed = false;
                    match key {
                        crate::event::Key::Backspace => {
                            if let Some(start) = self.selection_start {
                                let x1 = start.min(self.cursor_index);
                                let x2 = start.max(self.cursor_index);
                                if x1 != x2 {
                                    self.text.replace_range(x1..x2, "");
                                    self.cursor_index = x1;
                                    changed = true;
                                }
                                self.selection_start = None;
                            } else if self.cursor_index > 0 {
                                if let Some((idx, _)) =
                                    self.text[..self.cursor_index].char_indices().next_back()
                                {
                                    self.text.remove(idx);
                                    self.cursor_index = idx;
                                    changed = true;
                                }
                            }
                        }
                        crate::event::Key::Delete => {
                            if let Some(start) = self.selection_start {
                                let x1 = start.min(self.cursor_index);
                                let x2 = start.max(self.cursor_index);
                                if x1 != x2 {
                                    self.text.replace_range(x1..x2, "");
                                    self.cursor_index = x1;
                                    changed = true;
                                }
                                self.selection_start = None;
                            } else if self.cursor_index < self.text.len() {
                                self.text.remove(self.cursor_index);
                                changed = true;
                            }
                        }
                        crate::event::Key::Left => {
                            if self.cursor_index > 0 {
                                if let Some((idx, _)) =
                                    self.text[..self.cursor_index].char_indices().next_back()
                                {
                                    self.cursor_index = idx;
                                }
                            }
                            if !modifiers.shift {
                                self.selection_start = None;
                            } else if self.selection_start.is_none() {
                            }
                        }
                        crate::event::Key::Right => {
                            if self.cursor_index < self.text.len() {
                                if let Some(c) = self.text[self.cursor_index..].chars().next() {
                                    self.cursor_index += c.len_utf8();
                                }
                            }
                            if !modifiers.shift {
                                self.selection_start = None;
                            }
                        }
                        crate::event::Key::Home => {
                            self.cursor_index = 0;
                            if !modifiers.shift {
                                self.selection_start = None;
                            }
                        }
                        crate::event::Key::End => {
                            self.cursor_index = self.text.len();
                            if !modifiers.shift {
                                self.selection_start = None;
                            }
                        }
                        crate::event::Key::A if modifiers.ctrl => {
                            self.selection_start = Some(0);
                            self.cursor_index = self.text.len();
                        }
                        _ => {}
                    }

                    if changed {
                        self.blink_timer = 0.0;
                        if let Some(on_input) = &self.on_input {
                            messages.push(on_input(self.text.clone(), self.cursor_index));
                        }
                    }
                    *consumed = true;
                }
            }
            _ => {}
        }
        messages
    }

    fn flex(&self) -> u32 {
        self.flex
    }
}
