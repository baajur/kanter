use crate::{
    shared::*,
    slot_view::{Side, SlotView},
};
use orbtk::prelude::*;

impl Default for MouseState {
    fn default() -> Self {
        Self::MouseUp
    }
}

#[derive(Default, AsAny)]
pub struct NodeState {
    pub title: String16,
    pub mouse_action: Option<MouseAction>,
    mouse_state: MouseState,
    pub builder: WidgetBuildContext,
    input_slot_container: Entity,
    output_slot_container: Entity,
}

impl State for NodeState {
    fn init(&mut self, _: &mut Registry, ctx: &mut Context) {
        self.input_slot_container = ctx
            .entity_of_child("input_slot_container")
            .expect("`input_slot_container` child could not be found.");
        self.output_slot_container = ctx
            .entity_of_child("output_slot_container")
            .expect("`output_slot_container` child could not be found.");

        self.set_up_slots(ctx);
    }

    fn update(&mut self, _: &mut Registry, ctx: &mut Context) {
        if let Some(mouse_action) = self.mouse_action {
            match mouse_action {
                MouseAction::MousePressed => {
                    if self.mouse_state == MouseState::MouseUp {
                        let entity = ctx.widget().entity();
                        ctx.parent_from_id("node_workspace")
                            .set("dragged_node", Some(entity));
                        self.mouse_state = MouseState::MouseDown;
                    }
                }
                MouseAction::MouseReleased => {
                    if self.mouse_state == MouseState::MouseDown {
                        self.mouse_state = MouseState::MouseUp;
                    }
                }
            }
        }
    }
}

impl NodeState {
    pub fn mouse_action(&mut self, mouse_action: MouseAction) {
        self.mouse_action = Some(mouse_action);
    }

    fn set_up_slots(&mut self, ctx: &mut Context) {
        for _ in 0..*ctx.widget().get::<usize>("slot_count_input") {
            let build_context = &mut ctx.build_context();

            let item = SlotView::create().side(Side::Input).build(build_context);

            build_context.append_child(self.input_slot_container, item);
        }
        for _ in 0..*ctx.widget().get::<usize>("slot_count_output") {
            let build_context = &mut ctx.build_context();

            let item = SlotView::create().side(Side::Output).build(build_context);

            build_context.append_child(self.output_slot_container, item);
        }
    }
}
