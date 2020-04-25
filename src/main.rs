use orbtk::prelude::*;

use self::main_state::*;
use self::main_view::*;

mod main_state;
mod main_view;

mod node_workspace_state;
mod node_workspace_view;

mod node_state;
mod node_view;

mod slot_state;
mod slot_view;

mod edge_state;
mod edge_view;

mod line_render_object;
mod line_state;
mod line_view;

fn main() {
    Application::new()
        .window(|ctx| {
            Window::create()
                .title("OrbTk - minimal example")
                .position((300., 300.))
                .size(1024., 768.)
                .resizeable(true)
                .child(MainView::create().build(ctx))
                .build(ctx)
        })
        .run();
}
