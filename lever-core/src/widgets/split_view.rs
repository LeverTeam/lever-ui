use crate::draw::DrawList;
use crate::event::FrameworkEvent;
use crate::layout::{Constraints, LayoutNode, LayoutResult};
use crate::types::{Point, Rect};
use crate::widget::Widget;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SplitAxis {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct SplitViewState {
    pub ratio: f32,
    pub is_dragging: bool,
}

impl Default for SplitViewState {
    fn default() -> Self {
        Self {
            ratio: 0.5,
            is_dragging: false,
        }
    }
}

pub struct SplitView<M> {
    pub id: String,
    pub axis: SplitAxis,
    pub first: Box<dyn Widget<M>>,
    pub second: Box<dyn Widget<M>>,
    pub initial_ratio: f32,
    pub handle_size: f32,
    pub min_first: f32,
    pub min_second: f32,
    pub on_resize: Option<Box<dyn Fn(f32) -> M>>,
}

impl<M> SplitView<M> {
    pub fn new(
        id: impl Into<String>,
        axis: SplitAxis,
        first: Box<dyn Widget<M>>,
        second: Box<dyn Widget<M>>,
    ) -> Self {
        Self {
            id: id.into(),
            axis,
            first,
            second,
            initial_ratio: 0.5,
            handle_size: 6.0,
            min_first: 50.0,
            min_second: 50.0,
            on_resize: None,
        }
    }

    pub fn with_ratio(mut self, ratio: f32) -> Self {
        self.initial_ratio = ratio;
        self
    }

    pub fn with_min_sizes(mut self, first: f32, second: f32) -> Self {
        self.min_first = first;
        self.min_second = second;
        self
    }

    pub fn on_resize<F>(mut self, callback: F) -> Self
    where
        F: Fn(f32) -> M + 'static,
    {
        self.on_resize = Some(Box::new(callback));
        self
    }

    fn get_ratio(&self) -> f32 {
        crate::state::get_state::<SplitViewState>(&self.id)
            .map(|s| s.ratio)
            .unwrap_or(self.initial_ratio)
    }

    fn safe_clamp(&self, available: f32, ratio: f32) -> f32 {
        let min = self.min_first;
        let max = (available - self.min_second).max(min);
        (available * ratio).clamp(min, max).min(available)
    }
}

impl<M: 'static> Widget<M> for SplitView<M> {
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
        let ratio = self.get_ratio();
        let size = constraints.max_size();

        match self.axis {
            SplitAxis::Horizontal => {
                let available = (size.width - self.handle_size).max(0.0);
                let first_width = self.safe_clamp(available, ratio);
                let second_width = (available - first_width).max(0.0);

                self.first.layout(
                    Constraints::tight(first_width, size.height),
                    &[],
                    text_system,
                    theme,
                );
                self.second.layout(
                    Constraints::tight(second_width, size.height),
                    &[],
                    text_system,
                    theme,
                );
            }
            SplitAxis::Vertical => {
                let available = (size.height - self.handle_size).max(0.0);
                let first_height = self.safe_clamp(available, ratio);
                let second_height = (available - first_height).max(0.0);

                self.first.layout(
                    Constraints::tight(size.width, first_height),
                    &[],
                    text_system,
                    theme,
                );
                self.second.layout(
                    Constraints::tight(size.width, second_height),
                    &[],
                    text_system,
                    theme,
                );
            }
        }

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
        let ratio = self.get_ratio();
        let state = crate::state::get_state::<SplitViewState>(&self.id).unwrap_or_default();

        match self.axis {
            SplitAxis::Horizontal => {
                let available = (rect.width - self.handle_size).max(0.0);
                let first_width = self.safe_clamp(available, ratio);

                let first_rect = Rect {
                    x: rect.x,
                    y: rect.y,
                    width: first_width,
                    height: rect.height,
                };
                let handle_rect = Rect {
                    x: rect.x + first_width,
                    y: rect.y,
                    width: self.handle_size,
                    height: rect.height,
                };
                let second_rect = Rect {
                    x: rect.x + first_width + self.handle_size,
                    y: rect.y,
                    width: (available - first_width).max(0.0),
                    height: rect.height,
                };

                self.first.draw(
                    first_rect,
                    draw_list,
                    text_system,
                    theme,
                    focused_id,
                    pointer_pos,
                );
                self.second.draw(
                    second_rect,
                    draw_list,
                    text_system,
                    theme,
                    focused_id,
                    pointer_pos,
                );

                let is_hovered =
                    pointer_pos.map_or(false, |p| handle_rect.inset(-2.0, 0.0).contains(p));
                let handle_color = if state.is_dragging {
                    theme.primary
                } else if is_hovered {
                    theme.primary.with_alpha(0.5)
                } else {
                    theme.border.with_alpha(0.5)
                };

                draw_list.colored_rect(handle_rect, handle_color, 0.0);
            }
            SplitAxis::Vertical => {
                let available = (rect.height - self.handle_size).max(0.0);
                let first_height = self.safe_clamp(available, ratio);

                let first_rect = Rect {
                    x: rect.x,
                    y: rect.y,
                    width: rect.width,
                    height: first_height,
                };
                let handle_rect = Rect {
                    x: rect.x,
                    y: rect.y + first_height,
                    width: rect.width,
                    height: self.handle_size,
                };
                let second_rect = Rect {
                    x: rect.x,
                    y: rect.y + first_height + self.handle_size,
                    width: rect.width,
                    height: (available - first_height).max(0.0),
                };

                self.first.draw(
                    first_rect,
                    draw_list,
                    text_system,
                    theme,
                    focused_id,
                    pointer_pos,
                );
                self.second.draw(
                    second_rect,
                    draw_list,
                    text_system,
                    theme,
                    focused_id,
                    pointer_pos,
                );

                let is_hovered =
                    pointer_pos.map_or(false, |p| handle_rect.inset(0.0, -2.0).contains(p));
                let handle_color = if state.is_dragging {
                    theme.primary
                } else if is_hovered {
                    theme.primary.with_alpha(0.5)
                } else {
                    theme.border.with_alpha(0.5)
                };

                draw_list.colored_rect(handle_rect, handle_color, 0.0);
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
        let mut state = crate::state::get_or_set_state(&self.id, || SplitViewState {
            ratio: self.initial_ratio,
            is_dragging: false,
        });

        if state.is_dragging {
            if let FrameworkEvent::PointerMove { position } = event {
                match self.axis {
                    SplitAxis::Horizontal => {
                        let available = (rect.width - self.handle_size).max(0.0);
                        let local_x = (position.x - rect.x).clamp(0.0, available);
                        state.ratio = if available > 0.0 {
                            local_x / available
                        } else {
                            0.0
                        };
                    }
                    SplitAxis::Vertical => {
                        let available = (rect.height - self.handle_size).max(0.0);
                        let local_y = (position.y - rect.y).clamp(0.0, available);
                        state.ratio = if available > 0.0 {
                            local_y / available
                        } else {
                            0.0
                        };
                    }
                }

                if let Some(on_resize) = &self.on_resize {
                    messages.push(on_resize(state.ratio));
                }

                crate::state::set_state(&self.id, state);
                *consumed = true;
                return messages;
            }

            if let FrameworkEvent::PointerUp { .. } = event {
                state.is_dragging = false;
                crate::state::set_state(&self.id, state);
                *consumed = true;
                return messages;
            }
        }

        let ratio = state.ratio;
        match self.axis {
            SplitAxis::Horizontal => {
                let available = (rect.width - self.handle_size).max(0.0);
                let first_width = self.safe_clamp(available, ratio);

                let handle_rect = Rect {
                    x: rect.x + first_width,
                    y: rect.y,
                    width: self.handle_size,
                    height: rect.height,
                };
                if let FrameworkEvent::PointerDown { position, .. } = event {
                    if handle_rect.inset(-4.0, 0.0).contains(*position) {
                        state.is_dragging = true;
                        crate::state::set_state(&self.id, state);
                        *consumed = true;
                        return messages;
                    }
                }

                let first_rect = Rect {
                    x: rect.x,
                    y: rect.y,
                    width: first_width,
                    height: rect.height,
                };
                messages.extend(self.first.on_event(
                    event,
                    first_rect,
                    text_system,
                    theme,
                    focused_id,
                    consumed,
                ));
                if *consumed {
                    return messages;
                }

                let second_rect = Rect {
                    x: rect.x + first_width + self.handle_size,
                    y: rect.y,
                    width: (available - first_width).max(0.0),
                    height: rect.height,
                };
                messages.extend(self.second.on_event(
                    event,
                    second_rect,
                    text_system,
                    theme,
                    focused_id,
                    consumed,
                ));
            }
            SplitAxis::Vertical => {
                let available = (rect.height - self.handle_size).max(0.0);
                let first_height = self.safe_clamp(available, ratio);

                let handle_rect = Rect {
                    x: rect.x,
                    y: rect.y + first_height,
                    width: rect.width,
                    height: self.handle_size,
                };
                if let FrameworkEvent::PointerDown { position, .. } = event {
                    if handle_rect.inset(0.0, -4.0).contains(*position) {
                        state.is_dragging = true;
                        crate::state::set_state(&self.id, state);
                        *consumed = true;
                        return messages;
                    }
                }

                let first_rect = Rect {
                    x: rect.x,
                    y: rect.y,
                    width: rect.width,
                    height: first_height,
                };
                messages.extend(self.first.on_event(
                    event,
                    first_rect,
                    text_system,
                    theme,
                    focused_id,
                    consumed,
                ));
                if *consumed {
                    return messages;
                }

                let second_rect = Rect {
                    x: rect.x,
                    y: rect.y + first_height + self.handle_size,
                    width: rect.width,
                    height: (available - first_height).max(0.0),
                };
                messages.extend(self.second.on_event(
                    event,
                    second_rect,
                    text_system,
                    theme,
                    focused_id,
                    consumed,
                ));
            }
        }

        messages
    }
}
