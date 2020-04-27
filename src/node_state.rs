use crate::{shared::*, slot_view::SlotView};
use orbtk::prelude::*;

#[derive(Default, AsAny)]
pub struct NodeState {
    pub title: String16,
    pub mouse_action: Option<MouseAction>,
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

    fn update_post_layout(&mut self, _: &mut Registry, ctx: &mut Context) {
        if let Some(mouse_action) = self.mouse_action {
            match mouse_action {
                MouseAction::MousePressed => {
                    let entity = ctx.widget().entity();
                    ctx.parent_from_id("node_workspace")
                        .set("dragged_entity", Some(WidgetType::Node(entity)));
                }
                MouseAction::MouseReleased => {}
            }
        }
        self.mouse_action = None;
    }
}

impl NodeState {
    pub fn mouse_action(&mut self, mouse_action: MouseAction) {
        self.mouse_action = Some(mouse_action);
    }

    fn set_up_slots(&mut self, ctx: &mut Context) {
        let node_id = *ctx.widget().get::<u32>("node_id");

        for i in 0..*ctx.widget().get::<usize>("slot_count_input") {
            let build_context = &mut ctx.build_context();

            let item = SlotView::create()
                .node_id(node_id)
                .side(WidgetSide::Input)
                .slot_id(i as u32)
                .build(build_context);

            build_context.append_child(self.input_slot_container, item);
        }

        for i in 0..*ctx.widget().get::<usize>("slot_count_output") {
            let build_context = &mut ctx.build_context();

            let item = SlotView::create()
                .node_id(node_id)
                .side(WidgetSide::Output)
                .slot_id(i as u32)
                .build(build_context);

            build_context.append_child(self.output_slot_container, item);
        }
    }
}
