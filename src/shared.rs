#[derive(Copy, Clone)]
pub enum MouseAction {
    MousePressed,
    MouseReleased,
}

#[derive(PartialEq)]
pub enum MouseState {
    MouseDown,
    MouseUp,
}
