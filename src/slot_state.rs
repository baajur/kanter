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
        if let Some(mouse_action) = self.mouse_action {
            match mouse_action {
                MouseAction::MousePressed => {
                    let entity = ctx.widget().entity();

                    ctx.parent_from_id("node_workspace")
                        .set::<OptionDragDropEntity>(
                            "dragged_entity",
                            Some(DragDropEntity::new(WidgetType::Slot, entity)),
                        );
                }
                MouseAction::MouseReleased => {
                    let mut bounds = *ctx.widget().get::<Rectangle>("bounds");
                    bounds.x = ctx.widget().get::<Point>("position").x;
                    bounds.y = ctx.widget().get::<Point>("position").y;

                    let mouse_pos = (self.mouse_position.x, self.mouse_position.y);

                    if bounds.contains(mouse_pos) {
                        let entity = ctx.widget().entity();
                        let mut node_workspace = ctx.parent_from_id("node_workspace");
                        // let node_workspace_entity = node_workspace.entity();

                        node_workspace.set("dropped_on_entity", Some((WidgetType::Slot, entity)));

                        // ctx.push_event_strategy(ChangedEvent(node_workspace_entity), EventStrategy::Direct);
                    }
                }
            }
        }
        self.mouse_action = None;
    }
}

impl SlotState {
    pub fn mouse_action(&mut self, mouse_action: MouseAction, pos: Point) {
        self.mouse_position = pos;
        self.mouse_action = Some(mouse_action);
    }
}
