use orbtk::prelude::*;

use self::main_state::*;
use self::main_view::*;

mod main_state;
mod main_view;

mod node_state;
mod node_view;

fn main() {
    Application::new()
       .window(|ctx| {
           Window::create()
               .title("OrbTk - minimal example")
               .position((700.0, 300.0))
               .size(1024.0, 768.0)
               .resizeable(true)
               .child(MainView::create().build(ctx))
               .build(ctx)
       })
       .run();
}