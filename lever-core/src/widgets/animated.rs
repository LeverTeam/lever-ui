use crate::draw::DrawList;
use crate::event::FrameworkEvent;
use crate::layout::{Constraints, LayoutNode, LayoutResult};
use crate::types::Rect;
use crate::widget::Widget;

pub struct AnimatedOpacity<M> {
    pub opacity: f32,
    pub child: Box<dyn Widget<M>>,
}

impl<M> AnimatedOpacity<M> {
    pub fn new(opacity: f32, child: Box<dyn Widget<M>>) -> Self {
        Self { opacity, child }
    }
}

impl<M: 'static> Widget<M> for AnimatedOpacity<M> {
    fn id(&self) -> Option<&str> {
        self.child.id()
    }

    fn layout(
        &self,
        constraints: Constraints,
        children: &[LayoutNode],
        text_system: &mut crate::text::TextSystem,
        theme: &crate::theme::Theme,
    ) -> LayoutResult {
        self.child.layout(constraints, children, text_system, theme)
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
        if self.opacity <= 0.0 {
            return;
        }

        if self.opacity < 1.0 {
            draw_list.push_opacity(self.opacity);
        }

        self.child
            .draw(rect, draw_list, text_system, theme, focused_id, pointer_pos);

        if self.opacity < 1.0 {
            draw_list.pop_opacity();
        }
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
        self.child
            .on_event(event, rect, text_system, theme, focused_id, consumed)
    }
}

pub struct AnimatedTranslation<M> {
    pub offset: crate::types::Point,
    pub child: Box<dyn Widget<M>>,
}

impl<M> AnimatedTranslation<M> {
    pub fn new(offset: crate::types::Point, child: Box<dyn Widget<M>>) -> Self {
        Self { offset, child }
    }
}

impl<M: 'static> Widget<M> for AnimatedTranslation<M> {
    fn id(&self) -> Option<&str> {
        self.child.id()
    }

    fn layout(
        &self,
        constraints: Constraints,
        children: &[LayoutNode],
        text_system: &mut crate::text::TextSystem,
        theme: &crate::theme::Theme,
    ) -> LayoutResult {
        self.child.layout(constraints, children, text_system, theme)
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
        draw_list.push_translation(self.offset);
        self.child
            .draw(rect, draw_list, text_system, theme, focused_id, pointer_pos);
        draw_list.pop_translation();
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
        let translated_rect = rect.translate(self.offset);
        self.child.on_event(
            event,
            translated_rect,
            text_system,
            theme,
            focused_id,
            consumed,
        )
    }
}

pub struct AnimatedScale<M> {
    pub scale: f32,
    pub child: Box<dyn Widget<M>>,
}

impl<M> AnimatedScale<M> {
    pub fn new(scale: f32, child: Box<dyn Widget<M>>) -> Self {
        Self { scale, child }
    }
}

impl<M: 'static> Widget<M> for AnimatedScale<M> {
    fn id(&self) -> Option<&str> {
        self.child.id()
    }

    fn layout(
        &self,
        constraints: Constraints,
        children: &[LayoutNode],
        text_system: &mut crate::text::TextSystem,
        theme: &crate::theme::Theme,
    ) -> LayoutResult {
        self.child.layout(constraints, children, text_system, theme)
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
        let pivot = crate::types::Point {
            x: rect.x + rect.width / 2.0,
            y: rect.y + rect.height / 2.0,
        };
        draw_list.push_scale(self.scale, pivot);
        self.child
            .draw(rect, draw_list, text_system, theme, focused_id, pointer_pos);
        draw_list.pop_scale();
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
        let scaled_rect = rect.scale_centered(self.scale);
        self.child
            .on_event(event, scaled_rect, text_system, theme, focused_id, consumed)
    }
}

pub struct AnimatedClip<M> {
    pub clip_rect: Rect,
    pub child: Box<dyn Widget<M>>,
}

impl<M> AnimatedClip<M> {
    pub fn new(clip_rect: Rect, child: Box<dyn Widget<M>>) -> Self {
        Self { clip_rect, child }
    }
}

impl<M: 'static> Widget<M> for AnimatedClip<M> {
    fn id(&self) -> Option<&str> {
        self.child.id()
    }

    fn layout(
        &self,
        constraints: Constraints,
        children: &[LayoutNode],
        text_system: &mut crate::text::TextSystem,
        theme: &crate::theme::Theme,
    ) -> LayoutResult {
        self.child.layout(constraints, children, text_system, theme)
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
        draw_list.clip_push(self.clip_rect);
        self.child
            .draw(rect, draw_list, text_system, theme, focused_id, pointer_pos);
        draw_list.clip_pop();
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
        if let Some(pos) = event.pointer_pos() {
            if !self.clip_rect.contains(pos) {
                return Vec::new();
            }
        }
        self.child
            .on_event(event, rect, text_system, theme, focused_id, consumed)
    }
}
