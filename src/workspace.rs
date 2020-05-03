use crate::{node_container::NodeContainer, shared::*};
use kanter_core::node::{MixType, NodeType};
use orbtk::{
    prelude::*,
    shell::{ButtonState, Key},
};
use std::cell::Cell;

widget!(Workspace<WorkspaceState>: MouseHandler, KeyDownHandler {
    action_main: OptionActionMain,
    focused: bool
});

impl Template for Workspace {
    fn template(mut self, id: Entity, ctx: &mut BuildContext) -> Self {
        let node_container = NodeContainer::create().build(ctx);
        self.state_mut().node_container = node_container;

        let menu_node = Popup::create()
            .width(200.)
            .target(id)
            .child(
                Stack::create()
                    .orientation("vertical")
                    .child(
                        Button::create()
                            .element("button")
                            .on_click(move |states, _| {
                                states
                                    .get_mut::<WorkspaceState>(id)
                                    .add_node(NodeType::Mix(MixType::default()));
                                true
                            })
                            .text("Mix")
                            .build(ctx),
                    )
                    .child(
                        Button::create()
                            .element("button")
                            .on_click(move |states, _| {
                                states
                                    .get_mut::<WorkspaceState>(id)
                                    .add_node(NodeType::Value(0.));
                                true
                            })
                            .text("Value")
                            .build(ctx),
                    )
                    .child(
                        Button::create()
                            .element("button")
                            .on_click(move |states, _| {
                                states
                                    .get_mut::<WorkspaceState>(id)
                                    .add_node(NodeType::Resize(None, None));
                                true
                            })
                            .text("Resize")
                            .build(ctx),
                    )
                    .child(
                        Button::create()
                            .element("button")
                            .on_click(move |states, _| {
                                states
                                    .get_mut::<WorkspaceState>(id)
                                    .add_node(NodeType::HeightToNormal);
                                true
                            })
                            .text("HeightToNormal")
                            .build(ctx),
                    )
                    .child(
                        Button::create()
                            .element("button")
                            .on_click(move |states, _| {
                                states
                                    .get_mut::<WorkspaceState>(id)
                                    .add_node(NodeType::InputGray);
                                true
                            })
                            .text("InputGray")
                            .build(ctx),
                    )
                    .child(
                        Button::create()
                            .element("button")
                            .on_click(move |states, _| {
                                states
                                    .get_mut::<WorkspaceState>(id)
                                    .add_node(NodeType::InputRgba);
                                true
                            })
                            .text("InputRgba")
                            .build(ctx),
                    )
                    .child(
                        Button::create()
                            .element("button")
                            .on_click(move |states, _| {
                                states
                                    .get_mut::<WorkspaceState>(id)
                                    .add_node(NodeType::OutputGray);
                                true
                            })
                            .text("OutputGray")
                            .build(ctx),
                    )
                    .child(
                        Button::create()
                            .element("button")
                            .on_click(move |states, _| {
                                states
                                    .get_mut::<WorkspaceState>(id)
                                    .add_node(NodeType::OutputRgba);
                                true
                            })
                            .text("OutputRgba")
                            .build(ctx),
                    )
                    .build(ctx),
            )
            .build(ctx);
        self.state_mut().menu_node = menu_node;

        self.name("Workspace")
            .on_mouse_move(move |states, p| {
                states.get::<WorkspaceState>(id).action(Action::Move(p));
                false
            })
            .on_mouse_down(move |states, m| {
                states.get::<WorkspaceState>(id).action(Action::Press(m));
                false
            })
            .on_mouse_up(move |states, m| {
                states.get::<WorkspaceState>(id).action(Action::Release(m));
                false
            })
            .on_key_down(move |states, event| -> bool {
                if event.key == Key::Delete && event.state == ButtonState::Down {
                    states.get_mut::<WorkspaceState>(id).action(Action::Delete);
                }
                false
            })
            .child(node_container)
            .child(menu_node)
    }
}

#[derive(Default, AsAny)]
struct WorkspaceState {
    action: Cell<OptionAction>,
    node_container: Entity,
    menu_node: Entity,
    add_node: OptionNodeType,
}

impl State for WorkspaceState {
    fn init(&mut self, _: &mut Registry, ctx: &mut Context<'_>) {
        ctx.parent()
            .set::<u32>("node_container_entity", self.node_container.0);
        ctx.push_event_by_window(FocusEvent::RequestFocus(ctx.entity));
    }

    fn update(&mut self, _: &mut Registry, ctx: &mut Context<'_>) {
        self.handle_action_main(ctx);
        self.propagate_action(ctx);
    }
}

impl WorkspaceState {
    fn add_node(&mut self, node_type: NodeType) {
        self.add_node = Some(node_type);
    }

    fn action(&self, action: Action) {
        self.action.set(Some(action));
    }

    fn handle_action_main(&mut self, ctx: &mut Context) {
        if let Some(action_main) = ctx.widget().get::<OptionActionMain>("action_main") {
            match action_main {
                ActionMain::MenuNode(_) => {
                    let mut menu_node_widget = ctx.get_widget(self.menu_node);
                    menu_node_widget.set::<bool>("open", true);
                    menu_node_widget.set::<Thickness>(
                        "margin",
                        Thickness {
                            left: 0.,
                            top: 20.,
                            right: 0.,
                            bottom: 0.,
                        },
                    );
                }
                _ => {}
            };
        }

        ctx.widget().set::<OptionActionMain>("action_main", None);
    }

    fn propagate_action(&mut self, ctx: &mut Context) {
        if self.add_node.is_some() {
            ctx.get_widget(self.node_container)
                .set::<OptionNodeType>("add_node", self.add_node.clone());
            self.add_node = None;
            ctx.get_widget(self.menu_node).set::<bool>("open", false);
        } else {
            ctx.get_widget(self.node_container)
                .set::<OptionAction>("action", self.action.get());
            self.action.set(None);
        }
    }
}
