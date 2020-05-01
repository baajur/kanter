use crate::{node_container::NodeContainer, shared::*};
use orbtk::prelude::*;
use std::cell::Cell;

widget!(Workspace<WorkspaceState>: MouseHandler {
});

impl Template for Workspace {
    fn template(mut self, id: Entity, ctx: &mut BuildContext) -> Self {
        let node_container = NodeContainer::create().build(ctx);
        self.state_mut().node_container = node_container.into();

        self.name("Workspace")
            .on_mouse_move(move |states, p| {
                states.get::<WorkspaceState>(id).action(Action::Move(p));
                false
            })
            .on_mouse_down(move |states, p| {
                states.get::<WorkspaceState>(id).action(Action::Press(p));
                false
            })
            .on_mouse_up(move |states, p| {
                states.get::<WorkspaceState>(id).action(Action::Release(p));
                false
            })
            .child(node_container)
    }
}

#[derive(Default, AsAny)]
struct WorkspaceState {
    action: Cell<OptionAction>,
    node_container: Entity,
}

impl State for WorkspaceState {
    fn init(&mut self, _: &mut Registry, ctx: &mut Context<'_>) {
        ctx.parent()
            .set::<u32>("node_container_entity", self.node_container.0);
    }

    fn update(&mut self, _: &mut Registry, ctx: &mut Context<'_>) {
        self.propagate_action(ctx);
    }
}

impl WorkspaceState {
    fn action(&self, action: Action) {
        self.action.set(Some(action));
    }

    fn propagate_action(&mut self, ctx: &mut Context) {
        ctx.get_widget(self.node_container)
            .set::<OptionAction>("action", self.action.get());
        self.action.set(None);
    }
}
