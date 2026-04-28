use crate::animated::{animated_color, animated_spring};
use crate::animation::Spring;
use crate::draw::DrawList;
use crate::event::{FrameworkEvent, PointerButton};
use crate::layout::{Constraints, LayoutNode, LayoutResult};
use crate::types::{Color, Point, Rect, Size};
use crate::widget::Widget;

pub struct RadioButton<M> {
    pub id: String,
    pub is_selected: bool,
    pub is_disabled: bool,
    pub label: Option<String>,
    pub on_selected: Option<Box<dyn Fn() -> M>>,
}

impl<M> RadioButton<M> {
    pub fn new(id: impl Into<String>, is_selected: bool) -> Self {
        Self {
            id: id.into(),
            is_selected,
            is_disabled: false,
            label: None,
            on_selected: None,
        }
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }

    pub fn on_selected<F>(mut self, f: F) -> Self
    where
        F: Fn() -> M + 'static,
    {
        self.on_selected = Some(Box::new(f));
        self
    }
}

impl<M: 'static> Widget<M> for RadioButton<M> {
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
        let mut width = 24.0;
        if let Some(label) = &self.label {
            let layout = text_system.shape(
                label,
                theme.font_size_md,
                Color::WHITE,
                None,
                crate::types::TextAlign::Left,
            );
            width += theme.padding_sm + layout.width;
        }

        let size = constraints.clamp_size(Size {
            width,
            height: 24.0,
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
        let is_hovered = !self.is_disabled && pointer_pos.map_or(false, |pos| rect.contains(pos));

        let circle_radius = 12.0;
        let circle_center = Point {
            x: rect.x + circle_radius,
            y: rect.y + circle_radius,
        };
        let circle_rect = Rect {
            x: rect.x,
            y: rect.y,
            width: 24.0,
            height: 24.0,
        };

        // Animate border color
        let target_border = if self.is_disabled {
            theme.border
        } else if self.is_selected {
            theme.primary
        } else if is_hovered {
            theme.text_muted
        } else {
            theme.border
        };
        let border_color = animated_color(&format!("{}_border", self.id), target_border, 0.15);

        // Draw outer circle
        draw_list.stroke_rect(circle_rect, border_color, circle_radius, 2.0);

        // Draw inner dot with spring animation
        let dot_scale = animated_spring(
            &format!("{}_dot_scale", self.id),
            if self.is_selected { 1.0 } else { 0.0 },
            Spring::SNAPPY,
        );

        if dot_scale > 0.01 {
            let dot_radius = 6.0 * dot_scale;
            let dot_rect = Rect {
                x: circle_center.x - dot_radius,
                y: circle_center.y - dot_radius,
                width: dot_radius * 2.0,
                height: dot_radius * 2.0,
            };

            draw_list.rounded_rect(dot_rect, theme.primary, dot_radius);
        }

        if let Some(label) = &self.label {
            let text_color = if self.is_disabled {
                theme.text_muted
            } else {
                theme.text
            };
            let layout = text_system.shape(
                label,
                theme.font_size_md,
                text_color,
                None,
                crate::types::TextAlign::Left,
            );
            draw_list.text(
                Point {
                    x: (rect.x + 24.0 + theme.padding_sm).round(),
                    y: (rect.y + (rect.height - layout.height) / 2.0).round(),
                },
                layout.glyphs,
            );
        }
    }

    fn on_event(
        &mut self,
        event: &FrameworkEvent,
        rect: Rect,
        _text_system: &mut crate::text::TextSystem,
        _theme: &crate::theme::Theme,
        _focused_id: &mut Option<String>,
        _consumed: &mut bool,
    ) -> Vec<M> {
        let mut messages = Vec::new();
        match event {
            FrameworkEvent::PointerDown { position, button } => {
                if !self.is_disabled
                    && *button == PointerButton::Primary
                    && rect.contains(*position)
                {
                    if !self.is_selected {
                        if let Some(on_selected) = &self.on_selected {
                            messages.push(on_selected());
                        }
                    }
                }
            }
            _ => {}
        }
        messages
    }
}

pub struct RadioOption<V> {
    pub label: String,
    pub value: V,
    pub disabled: bool,
}

impl<V> RadioOption<V> {
    pub fn new(label: impl Into<String>, value: V) -> Self {
        Self {
            label: label.into(),
            value,
            disabled: false,
        }
    }

    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

pub struct RadioGroup<M, V> {
    pub id: String,
    pub options: Vec<RadioOption<V>>,
    pub selected_value: Option<usize>,
    pub on_changed: Option<std::sync::Arc<dyn Fn(V) -> M>>,
    pub direction: crate::layout::FlexDirection,
    pub gap: f32,
}

impl<M, V: Clone + 'static> RadioGroup<M, V> {
    pub fn new(id: impl Into<String>, options: Vec<RadioOption<V>>) -> Self {
        Self {
            id: id.into(),
            options,
            selected_value: None,
            on_changed: None,
            direction: crate::layout::FlexDirection::Column,
            gap: 8.0,
        }
    }

    pub fn with_selected(mut self, index: usize) -> Self {
        self.selected_value = Some(index);
        self
    }

    pub fn with_direction(mut self, direction: crate::layout::FlexDirection) -> Self {
        self.direction = direction;
        self
    }

    pub fn with_gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }

    pub fn on_changed<F>(mut self, f: F) -> Self
    where
        F: Fn(V) -> M + 'static,
    {
        self.on_changed = Some(std::sync::Arc::new(f));
        self
    }
}

impl<M: 'static, V: Clone + 'static> Widget<M> for RadioGroup<M, V> {
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
        let mut flex: crate::widgets::Flex<M> = match self.direction {
            crate::layout::FlexDirection::Row => crate::widgets::Flex::row(vec![]),
            crate::layout::FlexDirection::Column => crate::widgets::Flex::column(vec![]),
        };
        flex.gap = self.gap;

        for (i, option) in self.options.iter().enumerate() {
            flex = flex.with_child(Box::new(
                RadioButton::new(format!("{}_{}", self.id, i), self.selected_value == Some(i))
                    .with_label(&option.label)
                    .with_disabled(option.disabled),
            ));
        }

        flex.layout(constraints, &[], text_system, theme)
    }

    fn draw(
        &self,
        rect: Rect,
        draw_list: &mut DrawList,
        text_system: &mut crate::text::TextSystem,
        theme: &crate::theme::Theme,
        focused_id: Option<&str>,
        pointer_pos: Option<crate::types::Point>,
    ) {
        let mut flex: crate::widgets::Flex<M> = match self.direction {
            crate::layout::FlexDirection::Row => crate::widgets::Flex::row(vec![]),
            crate::layout::FlexDirection::Column => crate::widgets::Flex::column(vec![]),
        };
        flex.gap = self.gap;

        for (i, option) in self.options.iter().enumerate() {
            flex = flex.with_child(Box::new(
                RadioButton::new(format!("{}_{}", self.id, i), self.selected_value == Some(i))
                    .with_label(&option.label)
                    .with_disabled(option.disabled),
            ));
        }

        flex.draw(rect, draw_list, text_system, theme, focused_id, pointer_pos);
    }

    fn on_event(
        &mut self,
        event: &FrameworkEvent,
        rect: Rect,
        text_system: &mut crate::text::TextSystem,
        theme: &crate::theme::Theme,
        focused_id: &mut Option<String>,
        consumed: &mut bool,
    ) -> Vec<M> {
        let mut flex: crate::widgets::Flex<M> = match self.direction {
            crate::layout::FlexDirection::Row => crate::widgets::Flex::row(vec![]),
            crate::layout::FlexDirection::Column => crate::widgets::Flex::column(vec![]),
        };
        flex.gap = self.gap;

        for (i, option) in self.options.iter().enumerate() {
            let option_value = option.value.clone();
            let mut rb =
                RadioButton::new(format!("{}_{}", self.id, i), self.selected_value == Some(i))
                    .with_label(&option.label)
                    .with_disabled(option.disabled);

            if let Some(on_changed) = &self.on_changed {
                let on_changed = on_changed.clone();
                rb = rb.on_selected(move || on_changed(option_value.clone()));
            }

            flex = flex.with_child(Box::new(rb));
        }

        let mut messages = flex.on_event(event, rect, text_system, theme, focused_id, consumed);

        if !*consumed && focused_id.as_deref() == Some(&self.id) {
            if let FrameworkEvent::KeyDown { key, .. } = event {
                let mut new_index = None;
                match key {
                    crate::event::Key::Up | crate::event::Key::Left => {
                        let current = self.selected_value.unwrap_or(0);
                        if current > 0 {
                            new_index = Some(current - 1);
                        }
                    }
                    crate::event::Key::Down | crate::event::Key::Right => {
                        let current = self.selected_value.unwrap_or(0);
                        if current < self.options.len() - 1 {
                            new_index = Some(current + 1);
                        }
                    }
                    _ => {}
                }

                if let Some(idx) = new_index {
                    if !self.options[idx].disabled {
                        if let Some(on_changed) = &self.on_changed {
                            messages.push(on_changed(self.options[idx].value.clone()));
                            *consumed = true;
                        }
                    }
                }
            }
        }

        messages
    }
}
