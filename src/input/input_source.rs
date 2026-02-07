use sdl2::keyboard::Scancode;
use sdl2::mouse::MouseButton;

/// A physical input that can be bound to a game action.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum InputSource {
    /// A keyboard scancode.
    Key(Scancode),
    /// A mouse button.
    Mouse(MouseButton),
}