use crate::draw::DrawList;
use crate::layout::{Constraints, LayoutNode, LayoutResult};
use crate::types::Rect;

/// The base trait for all UI components in Lever.
///
/// A widget is responsible for defining its layout, rendering its visual state
/// to a [`DrawList`], and handling user input events.
pub trait Widget<M> {
    /// Builds a list of child widgets.
    ///
    /// This is used for composition-based widgets that don't implement
    /// their own layout or draw logic but instead wrap other widgets.
    fn build(&self) -> Vec<Box<dyn Widget<M>>> {
        Vec::new()
    }

    /// Calculates the size and internal layout of the widget based on the given constraints.
    ///
    /// # Arguments
    ///
    /// * `constraints` - The min/max size restrictions from the parent.
    /// * `children` - Pre-calculated layout nodes for children (if applicable).
    /// * `text_system` - Access to the text shaping engine.
    /// * `theme` - The current active theme.
    fn layout(
        &self,
        constraints: Constraints,
        children: &[LayoutNode],
        text_system: &mut crate::text::TextSystem,
        theme: &crate::theme::Theme,
    ) -> LayoutResult;

    /// Draws the widget's visual elements to the provided [`DrawList`].
    ///
    /// # Arguments
    ///
    /// * `rect` - The absolute bounding box where the widget should be drawn.
    /// * `draw_list` - The command buffer to push draw calls to.
    /// * `text_system` - Access to the text shaping engine.
    /// * `theme` - The current active theme.
    /// * `focused_id` - The ID of the currently focused widget, if any.
    /// * `pointer_pos` - The current mouse/touch position.
    fn draw(
        &self,
        rect: Rect,
        draw_list: &mut DrawList,
        text_system: &mut crate::text::TextSystem,
        theme: &crate::theme::Theme,
        focused_id: Option<&str>,
        pointer_pos: Option<crate::types::Point>,
    );

    /// Handles a user input or framework event.
    ///
    /// # Returns
    ///
    /// A vector of messages to be processed by the application's update loop.
    fn on_event(
        &mut self,
        _event: &crate::event::FrameworkEvent,
        _rect: Rect,
        _text_system: &mut crate::text::TextSystem,
        _theme: &crate::theme::Theme,
        _focused_id: &mut Option<String>,
    ) -> Vec<M> {
        Vec::new()
    }

    /// Returns the flex factor of the widget.
    ///
    /// A value of 0 means the widget is not flexible. Positive values indicate
    /// the relative weight of the widget when distributed within a flex container.
    fn flex(&self) -> u32 {
        0
    }

    /// Returns the unique identifier of the widget, if it has one.
    ///
    /// IDs are used for focus tracking and state preservation.
    fn id(&self) -> Option<&str> {
        None
    }

    /// Returns the absolute positioning offset of the widget, if any.
    ///
    /// This is used by [`Stack`] to place children at specific coordinates
    /// relative to the stack's bounds.
    fn positioned(&self) -> Option<crate::types::PositionedOffset> {
        None
    }

    /// Called every frame. Used for widget-local animations or state updates.
    fn tick(&mut self, _dt: f32) {}
}
