use crate::animated::{animated_color, animated_spring};
use crate::animation::Spring;
use crate::draw::DrawList;
use crate::event::{FrameworkEvent, PointerButton};
use crate::layout::{Constraints, LayoutNode, LayoutResult, MainAxisAlignment};
use crate::types::{Point, Rect, Size, TextureId};
use crate::widget::Widget;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum TabStyle {
    #[default]
    Underline,
    Pill,
    Ghost,
}

pub struct TabItem {
    pub label: String,
    pub icon: Option<TextureId>,
    pub disabled: bool,
}

impl TabItem {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            icon: None,
            disabled: false,
        }
    }

    pub fn with_icon(mut self, icon: TextureId) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

pub struct Tabs<M> {
    pub id: String,
    pub items: Vec<TabItem>,
    pub active_index: usize,
    pub style: TabStyle,
    pub alignment: MainAxisAlignment,
    pub full_width: bool,
    pub on_change: Option<Box<dyn Fn(usize) -> M>>,
}

impl<M> Tabs<M> {
    pub fn new(id: impl Into<String>, items: Vec<TabItem>, active_index: usize) -> Self {
        Self {
            id: id.into(),
            items,
            active_index,
            style: TabStyle::default(),
            alignment: MainAxisAlignment::Start,
            full_width: false,
            on_change: None,
        }
    }

    pub fn with_style(mut self, style: TabStyle) -> Self {
        self.style = style;
        self
    }

    pub fn with_alignment(mut self, alignment: MainAxisAlignment) -> Self {
        self.alignment = alignment;
        self
    }

    pub fn with_full_width(mut self, full_width: bool) -> Self {
        self.full_width = full_width;
        self
    }

    pub fn on_change<F>(mut self, f: F) -> Self
    where
        F: Fn(usize) -> M + 'static,
    {
        self.on_change = Some(Box::new(f));
        self
    }
}

impl<M: 'static> Widget<M> for Tabs<M> {
    fn id(&self) -> Option<&str> {
        Some(&self.id)
    }

    fn layout(
        &self,
        constraints: Constraints,
        _children: &[LayoutNode],
        text_system: &mut crate::text::TextSystem,
        theme: &crate::theme::Theme,
    ) -> LayoutResult {
        let mut total_width = 0.0;
        let font_size = 14.0;

        for item in &self.items {
            let layout = text_system.shape(
                &item.label,
                font_size,
                theme.text,
                None,
                crate::types::TextAlign::Left,
            );
            let mut item_width = layout.width + 32.0;
            if item.icon.is_some() {
                item_width += 20.0; // Icon + Gap
            }
            total_width += item_width;
        }

        let size = constraints.clamp_size(Size {
            width: if self.full_width && constraints.max_width.is_finite() {
                constraints.max_width
            } else {
                total_width
            },
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
        _focused_id: Option<&str>,
        pointer_pos: Option<crate::types::Point>,
    ) {
        let font_size = 14.0;
        let mut tab_widths = Vec::with_capacity(self.items.len());
        let mut total_content_width = 0.0;

        // First pass: measure
        for item in &self.items {
            let layout = text_system.shape(
                &item.label,
                font_size,
                theme.text,
                None,
                crate::types::TextAlign::Left,
            );
            let mut w = layout.width + 32.0;
            if item.icon.is_some() {
                w += 20.0;
            }
            tab_widths.push(w);
            total_content_width += w;
        }

        // Calculate starting X and individual widths
        let (mut current_x, draw_widths) = match self.alignment {
            MainAxisAlignment::SpaceBetween
            | MainAxisAlignment::SpaceAround
            | MainAxisAlignment::SpaceEvenly => {
                let w = rect.width / self.items.len() as f32;
                (rect.x, vec![w; self.items.len()])
            }
            MainAxisAlignment::Center => {
                let start_x = rect.x + (rect.width - total_content_width) / 2.0;
                (start_x, tab_widths)
            }
            MainAxisAlignment::End => {
                let start_x = rect.x + rect.width - total_content_width;
                (start_x, tab_widths)
            }
            MainAxisAlignment::Start => (rect.x, tab_widths),
        };

        let mut tab_rects = Vec::with_capacity(self.items.len());

        // Draw background for Contained style if needed
        if self.style == TabStyle::Pill {
            // Draw a subtle background track for the pill
            draw_list.rounded_rect(
                rect,
                theme.surface_variant.with_alpha(0.3),
                rect.height / 2.0,
            );
        }

        // Draw items
        for (i, item) in self.items.iter().enumerate() {
            let tab_width = draw_widths[i];
            let tab_rect = Rect {
                x: current_x,
                y: rect.y,
                width: tab_width,
                height: rect.height,
            };
            tab_rects.push(tab_rect);

            let is_active = i == self.active_index;
            let is_hovered = pointer_pos.map_or(false, |p| tab_rect.contains(p)) && !item.disabled;

            let base_color = if item.disabled {
                theme.text_muted.with_alpha(0.4)
            } else if is_active {
                theme.primary
            } else if is_hovered {
                theme.text
            } else {
                theme.text_muted
            };

            let text_color =
                animated_color(&format!("{}_tab_text_{}", self.id, i), base_color, 0.15);

            let layout = text_system.shape(
                &item.label,
                font_size,
                text_color,
                None,
                crate::types::TextAlign::Left,
            );

            let mut content_width = layout.width;
            if item.icon.is_some() {
                content_width += 20.0;
            }

            let mut content_x = current_x + (tab_width - content_width) / 2.0;
            let content_y = (rect.y + (rect.height - layout.height) / 2.0).round();

            // Draw icon
            if let Some(icon) = item.icon {
                draw_list.textured_rect(
                    Rect {
                        x: content_x.round(),
                        y: (rect.y + (rect.height - 16.0) / 2.0).round(),
                        width: 16.0,
                        height: 16.0,
                    },
                    icon,
                    text_color,
                    [0.0, 0.0, 1.0, 1.0],
                );
                content_x += 20.0;
            }

            // Draw text
            draw_list.text(
                Point {
                    x: content_x.round(),
                    y: content_y,
                },
                layout.glyphs,
            );

            current_x += tab_width;
        }

        // Animate indicator
        if let Some(active_rect) = tab_rects.get(self.active_index) {
            let indicator_x = animated_spring(
                &format!("{}_indicator_x", self.id),
                active_rect.x,
                Spring::SNAPPY,
            );
            let indicator_width = animated_spring(
                &format!("{}_indicator_w", self.id),
                active_rect.width,
                Spring::SNAPPY,
            );

            match self.style {
                TabStyle::Underline => {
                    let indicator_rect = Rect {
                        x: indicator_x,
                        y: rect.y + rect.height - 3.0,
                        width: indicator_width,
                        height: 3.0,
                    };
                    draw_list.rounded_rect(indicator_rect, theme.primary, 1.5);
                }
                TabStyle::Pill => {
                    let indicator_rect = Rect {
                        x: indicator_x,
                        y: rect.y + 4.0,
                        width: indicator_width,
                        height: rect.height - 8.0,
                    };
                    draw_list.rounded_rect(
                        indicator_rect,
                        theme.primary.with_alpha(0.15),
                        indicator_rect.height / 2.0,
                    );
                }
                TabStyle::Ghost => {
                    // No physical indicator, just text color (handled in loop)
                }
            }
        }
    }

    fn on_event(
        &mut self,
        event: &FrameworkEvent,
        rect: Rect,
        text_system: &mut crate::text::TextSystem,
        theme: &crate::theme::Theme,
        _focused_id: &mut Option<String>,
        consumed: &mut bool,
    ) -> Vec<M> {
        let mut messages = Vec::new();
        match event {
            FrameworkEvent::PointerDown { position, button } => {
                if *button == PointerButton::Primary && rect.contains(*position) {
                    let font_size = 14.0;
                    let mut tab_widths = Vec::with_capacity(self.items.len());
                    let mut total_content_width = 0.0;

                    for item in &self.items {
                        let layout = text_system.shape(
                            &item.label,
                            font_size,
                            theme.text,
                            None,
                            crate::types::TextAlign::Left,
                        );
                        let mut w = layout.width + 32.0;
                        if item.icon.is_some() {
                            w += 20.0;
                        }
                        tab_widths.push(w);
                        total_content_width += w;
                    }

                    let (mut current_x, draw_widths) = match self.alignment {
                        MainAxisAlignment::SpaceBetween
                        | MainAxisAlignment::SpaceAround
                        | MainAxisAlignment::SpaceEvenly => {
                            let w = rect.width / self.items.len() as f32;
                            (rect.x, vec![w; self.items.len()])
                        }
                        MainAxisAlignment::Center => {
                            let start_x = rect.x + (rect.width - total_content_width) / 2.0;
                            (start_x, tab_widths)
                        }
                        MainAxisAlignment::End => {
                            let start_x = rect.x + rect.width - total_content_width;
                            (start_x, tab_widths)
                        }
                        MainAxisAlignment::Start => (rect.x, tab_widths),
                    };

                    for (i, item) in self.items.iter().enumerate() {
                        let tab_width = draw_widths[i];
                        let tab_rect = Rect {
                            x: current_x,
                            y: rect.y,
                            width: tab_width,
                            height: rect.height,
                        };

                        if tab_rect.contains(*position) {
                            if !item.disabled {
                                *consumed = true;
                                if i != self.active_index {
                                    if let Some(on_change) = &self.on_change {
                                        messages.push(on_change(i));
                                    }
                                }
                            }
                            break;
                        }
                        current_x += tab_width;
                    }
                }
            }
            _ => {}
        }
        messages
    }
}
