use crate::animation::AnimationController;
use crate::draw::DrawList;
use crate::event::{FrameworkEvent, PointerButton};
use crate::layout::{Constraints, LayoutNode, LayoutResult};
use crate::state::get_or_set_state;
use crate::types::{Color, Rect, Size};
use crate::widget::Widget;

pub struct Toggle<M> {
    pub id: String,
    pub is_on: bool,
    pub on_changed: Option<Box<dyn Fn(bool) -> M>>,
}

impl<M> Toggle<M> {
    pub fn new(id: impl Into<String>, is_on: bool) -> Self {
        Self {
            id: id.into(),
            is_on,
            on_changed: None,
        }
    }

    pub fn on_changed<F>(mut self, f: F) -> Self
    where
        F: Fn(bool) -> M + 'static,
    {
        self.on_changed = Some(Box::new(f));
        self
    }
}

impl<M: 'static> Widget<M> for Toggle<M> {
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
            width: 44.0,
            height: 24.0,
        });
        LayoutResult { size }
    }

    fn draw(
        &self,
        rect: Rect,
        draw_list: &mut DrawList,
        _text_system: &mut crate::text::TextSystem,
        theme: &crate::theme::Theme,
        _focused_id: Option<&str>,
        _pointer_pos: Option<crate::types::Point>,
    ) {
        let anim = get_or_set_state(&self.id, || {
            AnimationController::new(if self.is_on { 1.0 } else { 0.0 })
        });
        let t = anim.value();

        let track_color = Color::lerp(theme.surface_variant, theme.success, t);
        draw_list.rounded_rect(rect, track_color, rect.height / 2.0);

        let thumb_radius = (rect.height - 4.0) / 2.0;
        let thumb_x = rect.x + 2.0 + t * (rect.width - rect.height);
        let thumb_y = rect.y + 2.0;

        draw_list.rounded_rect(
            Rect {
                x: thumb_x,
                y: thumb_y,
                width: rect.height - 4.0,
                height: rect.height - 4.0,
            },
            Color::rgb(1.0, 1.0, 1.0),
            thumb_radius,
        );
    }

    fn on_event(
        &mut self,
        event: &FrameworkEvent,
        rect: Rect,
        _text_system: &mut crate::text::TextSystem,
        _theme: &crate::theme::Theme,
        focused_id: &mut Option<String>,
        consumed: &mut bool,
    ) -> Vec<M> {
        let mut messages = Vec::new();
        match event {
            FrameworkEvent::PointerDown { position, button } => {
                if *button == PointerButton::Primary && rect.contains(*position) {
                    *consumed = true;
                    *focused_id = Some(self.id.clone());
                    self.is_on = !self.is_on;

                    crate::state::update_state::<AnimationController, _>(&self.id, |anim| {
                        anim.set_target(if self.is_on { 1.0 } else { 0.0 });
                    });

                    if let Some(on_changed) = &self.on_changed {
                        messages.push(on_changed(self.is_on));
                    }
                }
            }
            _ => {}
        }
        messages
    }
}
