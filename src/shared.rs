use orbtk::prelude::*;

#[derive(Copy, Clone)]
pub enum MouseAction {
    MousePressed,
    MouseReleased,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum WidgetType {
    Node(Entity),
    Slot(Entity),
    Edge,
}
into_property_source!(WidgetType);

impl Default for WidgetType {
    fn default() -> Self {
        Self::Edge
    }
}
