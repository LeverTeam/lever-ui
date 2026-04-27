use crate::animated::animated_color;
use crate::draw::DrawList;
use crate::event::{FrameworkEvent, PointerButton};
use crate::layout::{Constraints, LayoutNode, LayoutResult};
use crate::state::{get_or_set_state, update_state};
use crate::types::{BoxShadow, Color, Point, Rect, Size};
use crate::widget::Widget;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Danger,
    Outline,
    Ghost,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonSize {
    Small,
    Medium,
    Large,
}

#[derive(Debug, Clone, Default)]
struct ButtonState {
    is_pressed: bool,
}

pub struct Button<M> {
    pub id: String,
    pub label: String,
    pub variant: ButtonVariant,
    pub size: ButtonSize,
    pub color: Option<Color>,
    pub flex: u32,
    pub on_click: Option<Box<dyn Fn() -> M>>,
}

impl<M> Button<M> {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            variant: ButtonVariant::Primary,
            size: ButtonSize::Medium,
            color: None,
            flex: 0,
            on_click: None,
        }
    }

    pub fn with_variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn with_size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    pub fn with_flex(mut self, flex: u32) -> Self {
        self.flex = flex;
        self
    }

    pub fn on_click<F>(mut self, f: F) -> Self
    where
        F: Fn() -> M + 'static,
    {
        self.on_click = Some(Box::new(f));
        self
    }
}

impl<M: 'static> Widget<M> for Button<M> {
    fn id(&self) -> Option<&str> {
        Some(&self.id)
    }

    fn layout(
        &self,
        constraints: Constraints,
        _children: &[LayoutNode],
        text_system: &mut crate::text::TextSystem,
        _theme: &crate::theme::Theme,
    ) -> LayoutResult {
        let font_size = match self.size {
            ButtonSize::Small => 12.0,
            ButtonSize::Medium => 14.0,
            ButtonSize::Large => 16.0,
        };

        let h_padding = match self.size {
            ButtonSize::Small => 16.0,
            ButtonSize::Medium => 24.0,
            ButtonSize::Large => 32.0,
        };

        let height = match self.size {
            ButtonSize::Small => 32.0,
            ButtonSize::Medium => 40.0,
            ButtonSize::Large => 48.0,
        };

        let text_layout = text_system.shape(&self.label, font_size, Color::WHITE, None);
        let size = constraints.clamp_size(Size {
            width: text_layout.width + h_padding * 2.0,
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
        _focused_id: Option<&str>,
        pointer_pos: Option<crate::types::Point>,
    ) {
        let state = get_or_set_state::<ButtonState, _>(&self.id, || ButtonState::default());
        let is_hovered = pointer_pos.map_or(false, |pos| rect.contains(pos));

        // Determine colors based on variant and state
        let (bg_color, text_color, border_color) = match self.variant {
            ButtonVariant::Primary => {
                let base = self.color.unwrap_or(theme.primary);
                let target = if state.is_pressed {
                    Color {
                        r: (base.r * 0.9).max(0.0),
                        g: (base.g * 0.9).max(0.0),
                        b: (base.b * 0.9).max(0.0),
                        a: base.a,
                    }
                } else if is_hovered {
                    theme.primary_hover
                } else {
                    base
                };
                (target, theme.on_primary, Color::TRANSPARENT)
            }
            ButtonVariant::Secondary => {
                let target = if state.is_pressed {
                    theme.surface_variant
                } else if is_hovered {
                    Color {
                        r: (theme.surface_variant.r * 1.2).min(1.0),
                        g: (theme.surface_variant.g * 1.2).min(1.0),
                        b: (theme.surface_variant.b * 1.2).min(1.0),
                        a: theme.surface_variant.a,
                    }
                } else {
                    theme.surface
                };
                (target, theme.text, theme.border)
            }
            ButtonVariant::Danger => {
                let base = theme.danger;
                let target = if state.is_pressed {
                    Color {
                        r: (base.r * 0.8).max(0.0),
                        g: (base.g * 0.8).max(0.0),
                        b: (base.b * 0.8).max(0.0),
                        a: base.a,
                    }
                } else if is_hovered {
                    Color {
                        r: (base.r * 1.1).min(1.0),
                        g: (base.g * 1.1).min(1.0),
                        b: (base.b * 1.1).min(1.0),
                        a: base.a,
                    }
                } else {
                    base
                };
                (target, Color::WHITE, Color::TRANSPARENT)
            }
            ButtonVariant::Outline => {
                let bg = if state.is_pressed {
                    theme.surface_variant
                } else if is_hovered {
                    Color::rgba(theme.primary.r, theme.primary.g, theme.primary.b, 0.1)
                } else {
                    Color::TRANSPARENT
                };
                let border = if is_hovered || state.is_pressed {
                    theme.primary
                } else {
                    theme.border
                };
                (bg, theme.primary, border)
            }
            ButtonVariant::Ghost => {
                let bg = if state.is_pressed {
                    theme.surface_variant
                } else if is_hovered {
                    Color::rgba(theme.primary.r, theme.primary.g, theme.primary.b, 0.1)
                } else {
                    Color::TRANSPARENT
                };
                (bg, theme.primary, Color::TRANSPARENT)
            }
        };

        // Snappy color transitions
        let animated_bg = animated_color(&format!("{}_bg", self.id), bg_color, 0.08);
        let animated_text = animated_color(&format!("{}_text", self.id), text_color, 0.08);
        let animated_border = animated_color(&format!("{}_border", self.id), border_color, 0.08);

        // Draw background
        draw_list.rounded_rect(rect, animated_bg, theme.radius_md);

        // Draw border if needed
        if animated_border.a > 0.01 {
            draw_list.stroke_rect(rect, animated_border, theme.radius_md, 1.5);
        }

        // Draw shadow for solid buttons
        if matches!(self.variant, ButtonVariant::Primary | ButtonVariant::Danger)
            && !state.is_pressed
        {
            draw_list.shadowed_rect(
                rect,
                animated_bg,
                theme.radius_md,
                BoxShadow {
                    offset: Point { x: 0.0, y: 2.0 },
                    blur: 6.0,
                    color: theme.shadow_color,
                },
            );
        }

        // Label
        let font_size = match self.size {
            ButtonSize::Small => 12.0,
            ButtonSize::Medium => 14.0,
            ButtonSize::Large => 16.0,
        };
        let layout = text_system.shape(&self.label, font_size, animated_text, None);
        let x = rect.x + (rect.width - layout.width) / 2.0;
        let y = rect.y + (rect.height - layout.height) / 2.0;

        draw_list.text(
            Point {
                x: x.round(),
                y: y.round(),
            },
            layout.glyphs,
        );
    }

    fn on_event(
        &mut self,
        event: &FrameworkEvent,
        rect: Rect,
        _text_system: &mut crate::text::TextSystem,
        _theme: &crate::theme::Theme,
        _focused_id: &mut Option<String>,
    ) -> Vec<M> {
        let mut messages = Vec::new();
        let state = get_or_set_state::<ButtonState, _>(&self.id, || ButtonState::default());

        match event {
            FrameworkEvent::PointerDown { position, button } => {
                if *button == PointerButton::Primary && rect.contains(*position) {
                    update_state::<ButtonState, _>(&self.id, |s| s.is_pressed = true);
                }
            }
            FrameworkEvent::PointerUp { position, button } => {
                if *button == PointerButton::Primary {
                    if state.is_pressed && rect.contains(*position) {
                        if let Some(on_click) = &self.on_click {
                            messages.push(on_click());
                        }
                    }
                    update_state::<ButtonState, _>(&self.id, |s| s.is_pressed = false);
                }
            }
            FrameworkEvent::PointerMove { position } => {
                if state.is_pressed && !rect.contains(*position) {
                    // Optional: cancel press if mouse leaves the button
                    // update_state::<ButtonState, _>(&self.id, |s| s.is_pressed = false);
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
