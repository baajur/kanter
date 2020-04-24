use orbtk::prelude::*;

#[derive(Copy, Clone)]
pub enum Action {
    NewNode,
    LoadGraph,
    SaveGraph,
}

#[derive(Default, AsAny)]
pub struct MainState {
    pub action: Option<Action>,
}

impl MainState {
    pub fn action(&mut self, action: impl Into<Option<Action>>) {
        self.action = action.into();
    }
}

impl State for MainState {
    fn update(&mut self, _: &mut Registry, ctx: &mut Context) {
        if let Some(action) = self.action {
            match action {
                Action::NewNode => {
                    *ctx.child("node_workspace").get_mut::<usize>("count") += 1;
                }
                Action::LoadGraph => {
                    let path = ctx.child("graph_path").clone::<String16>("text");
                    ctx.child("node_workspace")
                        .set::<String16>("path_load", path);
                }
                Action::SaveGraph => {
                    let path = ctx.child("graph_path").clone::<String16>("text");
                    ctx.child("node_workspace")
                        .set::<String16>("path_save", path);
                }
            }
            self.action = None;
        }
    }
}
