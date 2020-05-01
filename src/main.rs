use crate::{shared::*, workspace::Workspace};
use orbtk::prelude::*;

mod edge;
mod line;
mod node;
mod node_container;
mod shared;
mod slot;
mod workspace;

fn main() {
    Application::new()
        .window(|ctx| {
            Window::create()
                .title("Kanter")
                .position((300., 300.))
                .size(1024., 768.)
                .resizeable(true)
                .child(MainView::create().build(ctx))
                .build(ctx)
        })
        .run();
}

widget!(MainView<MainState> {
    node_container_entity: u32
});

#[derive(Default, AsAny)]
pub struct MainState {
    pub option_action_file: OptionActionFile,
}

impl MainState {
    pub fn action_file(&mut self, option_action_file: impl Into<OptionActionFile>) {
        self.option_action_file = option_action_file.into();
    }
}

impl State for MainState {
    fn update(&mut self, _: &mut Registry, ctx: &mut Context) {
        if let Some(action_file) = &self.option_action_file {
            let path = ctx.child("graph_path").clone::<String16>("text");

            let action_to_send = match action_file {
                ActionFile::LoadGraph(_) => Some(ActionFile::LoadGraph(path.to_string())),
                ActionFile::SaveGraph(_) => Some(ActionFile::SaveGraph(path.to_string())),
            };

            let node_container_entity = Entity(*ctx.widget().get::<u32>("node_container_entity"));

            ctx.get_widget(node_container_entity)
                .set::<OptionActionFile>("action_file", action_to_send);

            self.option_action_file = None;
        }
    }
}

impl Template for MainView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("MainView")
            .child(Workspace::create().build(ctx))
            .child(
                Stack::create()
                    .orientation(Orientation::Horizontal)
                    .child(
                        Button::create()
                            .element("button")
                            .on_click(move |states, _| {
                                states
                                    .get_mut::<MainState>(id)
                                    .action_file(ActionFile::LoadGraph("".to_string()));
                                true
                            })
                            .text("Load graph")
                            .width(100.)
                            .build(ctx),
                    )
                    .child(
                        Button::create()
                            .element("button")
                            .on_click(move |states, _| {
                                states
                                    .get_mut::<MainState>(id)
                                    .action_file(ActionFile::SaveGraph("".to_string()));
                                true
                            })
                            .text("Save graph")
                            .width(100.)
                            .build(ctx),
                    )
                    .child(
                        TextBox::create()
                            .id("graph_path")
                            .text("data/invert_graph.json")
                            .build(ctx),
                    )
                    .build(ctx),
            )
    }
}
