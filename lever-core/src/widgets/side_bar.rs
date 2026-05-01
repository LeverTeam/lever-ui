use crate::animated::animated_spring;
use crate::animation::Spring;
use crate::draw::DrawList;
use crate::event::FrameworkEvent;
use crate::layout::{Constraints, LayoutNode, LayoutResult};
use crate::types::{Point, Rect};
use crate::widget::Widget;

pub struct SideBar<M> {
    pub id: String,
    pub sidebar: Box<dyn Widget<M>>,
    pub content: Box<dyn Widget<M>>,
    pub sidebar_width: f32,
    pub is_collapsed: bool,
}

impl<M> SideBar<M> {
    pub fn new(
        id: impl Into<String>,
        sidebar: Box<dyn Widget<M>>,
        content: Box<dyn Widget<M>>,
    ) -> Self {
        Self {
            id: id.into(),
            sidebar,
            content,
            sidebar_width: 260.0,
            is_collapsed: false,
        }
    }

    pub fn with_width(mut self, width: f32) -> Self {
        self.sidebar_width = width;
        self
    }

    pub fn with_collapsed(mut self, collapsed: bool) -> Self {
        self.is_collapsed = collapsed;
        self
    }
}

impl<M: 'static> Widget<M> for SideBar<M> {
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
        let size = constraints.max_size();
        let target_width = if self.is_collapsed {
            0.0
        } else {
            self.sidebar_width
        };
        let animated_width =
            animated_spring(&format!("{}_width", self.id), target_width, Spring::SMOOTH);

        if animated_width > 0.0 {
            self.sidebar.layout(
                Constraints::tight(animated_width, size.height),
                &[],
                text_system,
                theme,
            );
        }

        self.content.layout(
            Constraints::tight(size.width - animated_width, size.height),
            &[],
            text_system,
            theme,
        );

        LayoutResult { size }
    }

    fn draw(
        &self,
        rect: Rect,
        draw_list: &mut DrawList,
        text_system: &mut crate::text::TextSystem,
        theme: &crate::theme::Theme,
        focused_id: Option<&str>,
        pointer_pos: Option<Point>,
    ) {
        let target_width = if self.is_collapsed {
            0.0
        } else {
            self.sidebar_width
        };
        let animated_width =
            animated_spring(&format!("{}_width", self.id), target_width, Spring::SMOOTH);

        if animated_width > 0.01 {
            let sidebar_rect = Rect {
                x: rect.x,
                y: rect.y,
                width: animated_width,
                height: rect.height,
            };

            draw_list.clip_push(sidebar_rect);
            self.sidebar.draw(
                Rect {
                    x: rect.x - (self.sidebar_width - animated_width),
                    y: rect.y,
                    width: self.sidebar_width,
                    height: rect.height,
                },
                draw_list,
                text_system,
                theme,
                focused_id,
                pointer_pos,
            );
            draw_list.clip_pop();

            draw_list.line(
                Point::new(rect.x + animated_width, rect.y),
                Point::new(rect.x + animated_width, rect.y + rect.height),
                1.0,
                theme.border.with_alpha(0.5),
            );
        }

        let content_rect = Rect {
            x: rect.x + animated_width,
            y: rect.y,
            width: rect.width - animated_width,
            height: rect.height,
        };
        self.content.draw(
            content_rect,
            draw_list,
            text_system,
            theme,
            focused_id,
            pointer_pos,
        );
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
        let mut messages = Vec::new();
        let target_width = if self.is_collapsed {
            0.0
        } else {
            self.sidebar_width
        };
        let animated_width =
            animated_spring(&format!("{}_width", self.id), target_width, Spring::SMOOTH);

        if animated_width > 0.01 {
            let sidebar_rect = Rect {
                x: rect.x,
                y: rect.y,
                width: animated_width,
                height: rect.height,
            };
            messages.extend(self.sidebar.on_event(
                event,
                sidebar_rect,
                text_system,
                theme,
                focused_id,
                consumed,
            ));
            if *consumed {
                return messages;
            }
        }

        let content_rect = Rect {
            x: rect.x + animated_width,
            y: rect.y,
            width: rect.width - animated_width,
            height: rect.height,
        };
        messages.extend(self.content.on_event(
            event,
            content_rect,
            text_system,
            theme,
            focused_id,
            consumed,
        ));

        messages
    }
}
