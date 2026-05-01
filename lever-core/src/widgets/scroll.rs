use crate::draw::DrawList;
use crate::event::FrameworkEvent;
use crate::layout::{Constraints, LayoutNode, LayoutResult};
use crate::types::{Point, Rect, Size};
use crate::widget::Widget;

#[derive(Debug, Clone, Copy)]
pub(crate) struct ScrollState {
    pub content_size: Size,
    pub is_dragging_v: bool,
    pub _is_dragging_h: bool,
}

impl Default for ScrollState {
    fn default() -> Self {
        Self {
            content_size: Size::new(0.0, 0.0),
            is_dragging_v: false,
            _is_dragging_h: false,
        }
    }
}

pub struct Scroll<M> {
    pub id: String,
    pub child: Box<dyn Widget<M>>,
    pub scroll_offset: Point,
    pub on_scroll: Option<Box<dyn Fn(Point) -> M>>,
    pub flex: u32,
    pub show_vertical: bool,
    pub show_horizontal: bool,
}

impl<M> Scroll<M> {
    pub fn new(id: impl Into<String>, child: Box<dyn Widget<M>>) -> Self {
        Self {
            id: id.into(),
            child,
            scroll_offset: Point { x: 0.0, y: 0.0 },
            on_scroll: None,
            flex: 0,
            show_vertical: true,
            show_horizontal: false,
        }
    }

    pub fn with_vertical(mut self, show: bool) -> Self {
        self.show_vertical = show;
        self
    }

    pub fn with_horizontal(mut self, show: bool) -> Self {
        self.show_horizontal = show;
        self
    }

    pub fn with_offset(mut self, offset: Point) -> Self {
        self.scroll_offset = offset;
        self
    }

    pub fn on_scroll<F>(mut self, callback: F) -> Self
    where
        F: Fn(Point) -> M + 'static,
    {
        self.on_scroll = Some(Box::new(callback));
        self
    }

    pub fn with_flex(mut self, flex: u32) -> Self {
        self.flex = flex;
        self
    }
}

impl<M: 'static> Widget<M> for Scroll<M> {
    fn layout(
        &self,
        constraints: Constraints,
        _children: &[LayoutNode],
        text_system: &mut crate::text::TextSystem,
        theme: &crate::theme::Theme,
    ) -> LayoutResult {
        let child_constraints = Constraints {
            min_width: constraints.min_width,
            max_width: constraints.max_width,
            min_height: 0.0,
            max_height: f32::INFINITY,
        };

        let child_layout = self
            .child
            .layout(child_constraints, &[], text_system, theme);

        crate::state::set_state(
            &self.id,
            ScrollState {
                content_size: child_layout.size,
                ..crate::state::get_state::<ScrollState>(&self.id).unwrap_or_default()
            },
        );

        LayoutResult {
            size: constraints.clamp_size(Size {
                width: constraints.max_width,
                height: constraints.max_height,
            }),
        }
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
        let state = crate::state::get_state::<ScrollState>(&self.id).unwrap_or_default();
        let content_size = state.content_size;

        draw_list.clip_push(rect);

        let child_rect = Rect {
            x: rect.x - self.scroll_offset.x,
            y: rect.y - self.scroll_offset.y,
            width: rect.width,
            height: f32::INFINITY,
        };

        self.child.draw(
            child_rect,
            draw_list,
            text_system,
            theme,
            focused_id,
            pointer_pos,
        );

        draw_list.clip_pop();

        if content_size.height > rect.height {
            let is_hovered = pointer_pos.map_or(false, |pos| rect.contains(pos));
            let is_active = is_hovered || state.is_dragging_v;

            let opacity = crate::animated::animated_spring(
                &format!("{}_sb_opacity", self.id),
                if is_active { 1.0 } else { 0.0 },
                crate::animation::Spring::SNAPPY,
            );

            let width_scale = crate::animated::animated_spring(
                &format!("{}_sb_width", self.id),
                if is_active { 1.0 } else { 0.6 },
                crate::animation::Spring::SNAPPY,
            );

            if opacity > 0.01 {
                let ratio = rect.height / content_size.height;
                let handle_height = (rect.height * ratio).max(20.0);
                let max_scroll = content_size.height - rect.height;
                let scroll_ratio = (self.scroll_offset.y / max_scroll).clamp(0.0, 1.0);
                let handle_y = rect.y + scroll_ratio * (rect.height - handle_height);

                let sb_width = 6.0 * width_scale;
                let sb_rect = Rect {
                    x: rect.x + rect.width - sb_width - 2.0,
                    y: handle_y,
                    width: sb_width,
                    height: handle_height,
                };

                let sb_color = theme.text.with_alpha(0.3 * opacity);
                draw_list.rounded_rect(sb_rect, sb_color, sb_width / 2.0);
            }
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
        let mut messages = Vec::new();
        let state = crate::state::get_state::<ScrollState>(&self.id).unwrap_or_default();
        let content_size = state.content_size;

        if state.is_dragging_v {
            if let FrameworkEvent::PointerMove { position } = event {
                let max_scroll = (content_size.height - rect.height).max(0.0);
                if max_scroll > 0.0 {
                    let ratio = rect.height / content_size.height;
                    let handle_height = (rect.height * ratio).max(20.0);
                    let track_height = rect.height - handle_height;

                    let local_y =
                        (position.y - rect.y - handle_height / 2.0).clamp(0.0, track_height);
                    let scroll_ratio = local_y / track_height;

                    self.scroll_offset.y = scroll_ratio * max_scroll;

                    if let Some(on_scroll) = &self.on_scroll {
                        messages.push(on_scroll(self.scroll_offset));
                    }
                }
                *consumed = true;
                return messages;
            }

            if let FrameworkEvent::PointerUp { .. } = event {
                crate::state::update_state(&self.id, |s: &mut ScrollState| s.is_dragging_v = false);
                *consumed = true;
                return messages;
            }
        }

        let child_rect = Rect {
            x: rect.x - self.scroll_offset.x,
            y: rect.y - self.scroll_offset.y,
            width: content_size.width,
            height: content_size.height,
        };

        messages.extend(self.child.on_event(
            event,
            child_rect,
            text_system,
            theme,
            focused_id,
            consumed,
        ));

        if *consumed {
            return messages;
        }

        if let FrameworkEvent::Scroll { position, delta } = event {
            if rect.contains(*position) {
                let (max_scroll_x, max_scroll_y) =
                    if content_size.width > 0.0 || content_size.height > 0.0 {
                        (
                            (content_size.width - rect.width).max(0.0),
                            (content_size.height - rect.height).max(0.0),
                        )
                    } else {
                        (f32::INFINITY, f32::INFINITY)
                    };

                self.scroll_offset.x = (self.scroll_offset.x + delta.x).clamp(0.0, max_scroll_x);
                self.scroll_offset.y = (self.scroll_offset.y + delta.y).clamp(0.0, max_scroll_y);

                if let Some(on_scroll) = &self.on_scroll {
                    messages.push(on_scroll(self.scroll_offset));
                }
                *consumed = true;
                return messages;
            }
        }

        if let FrameworkEvent::PointerDown { position, button } = event {
            if *button == crate::event::PointerButton::Primary && rect.contains(*position) {
                if content_size.height > rect.height {
                    let sb_width = 12.0;
                    let sb_area = Rect {
                        x: rect.x + rect.width - sb_width,
                        y: rect.y,
                        width: sb_width,
                        height: rect.height,
                    };

                    if sb_area.contains(*position) {
                        crate::state::update_state(&self.id, |s: &mut ScrollState| {
                            s.is_dragging_v = true;
                        });
                        *focused_id = Some(self.id.clone());

                        let max_scroll = content_size.height - rect.height;
                        let ratio = rect.height / content_size.height;
                        let handle_height = (rect.height * ratio).max(20.0);
                        let track_height = rect.height - handle_height;

                        let local_y =
                            (position.y - rect.y - handle_height / 2.0).clamp(0.0, track_height);
                        let scroll_ratio = local_y / track_height;

                        self.scroll_offset.y = scroll_ratio * max_scroll;

                        if let Some(on_scroll) = &self.on_scroll {
                            messages.push(on_scroll(self.scroll_offset));
                        }

                        *consumed = true;
                        return messages;
                    }
                }
            }
        }

        messages
    }

    fn flex(&self) -> u32 {
        self.flex
    }
}
