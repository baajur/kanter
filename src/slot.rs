use crate::shared::*;
use orbtk::{behaviors::MouseBehavior, prelude::*};

widget!(
    Slot<SlotState>: MouseHandler {
        widget_type: WidgetType,
        side: WidgetSide,
        node_workspace: Entity,
        node_id: u32,
        slot_id: u32
    }
);

impl Template for Slot {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        // let margin_left = match self.side.as_ref().unwrap() {
        //     PropertySource::Value(WidgetSide::Input) => -SLOT_SIZE_HALF,
        //     PropertySource::Value(WidgetSide::Output) => NODE_SIZE - SLOT_SIZE_HALF,
        //     _ => {
        //         panic!("WidgetSide is not properly set");
        //     }
        // };

        self.name("Slot")
            .widget_type(WidgetType::Slot)
            .width(SLOT_SIZE)
            .height(SLOT_SIZE)
            .on_mouse_down(move |states, p| {
                states
                    .get_mut::<SlotState>(id)
                    .mouse_action(MouseAction::MousePressed, p);
                false
            })
            .on_global_mouse_up(move |states, p| {
                states
                    .get_mut::<SlotState>(id)
                    .mouse_action(MouseAction::MouseReleased, p);
            })
            .child(MouseBehavior::create().enabled(id).target(id.0).build(ctx))
            .child(
                Container::create()
                    .background(Color::rgb(200, 200, 200))
                    .border_width(1.)
                    .border_radius(SLOT_SIZE_HALF)
                    .border_brush(Brush::SolidColor(Color::rgb(0, 0, 0)))
                    .build(ctx),
            )
    }
}

use crate::shared::*;
use orbtk::prelude::*;

#[derive(AsAny)]
pub struct SlotState {
    pub mouse_action: Option<MouseAction>,
    mouse_position: Point,
}

impl Default for SlotState {
    fn default() -> Self {
        Self {
            mouse_action: None,
            mouse_position: Point { x: 0., y: 0. },
        }
    }
}

impl State for SlotState {
    fn update_post_layout(&mut self, _: &mut Registry, ctx: &mut Context) {
        // if let Some(mouse_action) = self.mouse_action {
        //     match mouse_action {
        //         MouseAction::MousePressed => {
        //             let entity = ctx.widget().entity();

        //             ctx.parent_from_id("node_workspace")
        //                 .set::<OptionDragDropEntity>(
        //                     "dragged_entity",
        //                     Some(DragDropEntity::new(WidgetType::Slot, entity)),
        //                 );
        //         }
        //         MouseAction::MouseReleased => {
        //             let mut bounds = *ctx.widget().get::<Rectangle>("bounds");
        //             bounds.x = ctx.widget().get::<Point>("position").x;
        //             bounds.y = ctx.widget().get::<Point>("position").y;

        //             let mouse_pos = (self.mouse_position.x, self.mouse_position.y);

        //             if bounds.contains(mouse_pos) {
        //                 let entity = ctx.widget().entity();
        //                 let mut node_workspace = ctx.parent_from_id("node_workspace");
        //                 // let node_workspace_entity = node_workspace.entity();

        //                 node_workspace.set::<OptionDragDropEntity>(
        //                     "dropped_on_entity",
        //                     Some(DragDropEntity::new(WidgetType::Slot, entity)),
        //                 );

        //                 // ctx.push_event_strategy(ChangedEvent(node_workspace_entity), EventStrategy::Direct);
        //             }
        //         }
        //     }
        // }
        // self.mouse_action = None;
    }
}

impl SlotState {
    pub fn mouse_action(&mut self, mouse_action: MouseAction, pos: Point) {
        self.mouse_position = pos;
        self.mouse_action = Some(mouse_action);
    }
}
