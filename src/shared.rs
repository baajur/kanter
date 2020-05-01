use kanter_core::node::{NodeType, Side};
use orbtk::prelude::*;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum MouseAction {
    MousePressed,
    MouseReleased,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum WidgetType {
    Node,
    Slot,
    Edge,
}
into_property_source!(WidgetType);

impl Default for WidgetType {
    fn default() -> Self {
        Self::Edge
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum WidgetSide {
    Input,
    Output,
}
into_property_source!(WidgetSide);

impl Default for WidgetSide {
    fn default() -> Self {
        Self::Input
    }
}

impl Into<Side> for WidgetSide {
    fn into(self) -> Side {
        match self {
            Self::Input => Side::Input,
            Self::Output => Side::Output,
        }
    }
}

#[derive(Copy, Clone, Debug, Default, AsAny)]
pub struct DragDropEntity {
    pub widget_type: WidgetType,
    pub entity: Entity,
}
into_property_source!(DragDropEntity);

impl DragDropEntity {
    pub fn new(widget_type: WidgetType, entity: Entity) -> Self {
        Self {
            widget_type,
            entity,
        }
    }
}

pub(crate) type OptionDragDropEntity = Option<DragDropEntity>;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Action {
    Press(Point),
    Release(Point),
    Scroll(Point),
    Move(Point),
}
pub type OptionAction = Option<Action>;

pub fn child_entities(ctx: &mut Context) -> Vec<Entity> {
    let mut output: Vec<Entity> = Vec::new();

    for i in 0.. {
        if let Some(child) = ctx.try_child_from_index(i) {
            output.push(child.entity());
        } else {
            break;
        }
    }

    output
}

#[derive(Debug, Clone)]
pub enum ActionMain {
    LoadGraph(String),
    SaveGraph(String),
    MenuNode(Point),
}
pub type OptionActionMain = Option<ActionMain>;

pub type OptionNodeType = Option<NodeType>;

pub const NODE_SIZE: f64 = 100.;
pub const SLOT_SIZE: f64 = 15.;
pub const SLOT_SIZE_HALF: f64 = SLOT_SIZE * 0.5;
pub const SLOT_SPACING: f64 = SLOT_SIZE_HALF;
