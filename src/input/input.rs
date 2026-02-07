use std::collections::HashSet;
use sdl2::keyboard::Scancode;
use sdl2::mouse::MouseButton;

/// Tracks keyboard and mouse state across frames for edge detection.
pub struct Input {
    current_keys: HashSet<Scancode>,
    previous_keys: HashSet<Scancode>,
    current_mouse: HashSet<MouseButton>,
    previous_mouse: HashSet<MouseButton>,
    mouse_delta: (f32, f32),
}

impl Input {
    /// Creates a new input tracker with no keys or buttons pressed.
    pub fn new() -> Self {
        Self {
            current_keys: HashSet::new(),
            previous_keys: HashSet::new(),
            previous_mouse: HashSet::new(),
            current_mouse: HashSet::new(),
            mouse_delta: (0.0, 0.0),
        }
    }

    /// Snapshots current state as previous and resets per-frame deltas. Called by the engine at end of frame.
    pub fn update(&mut self) {
        std::mem::swap(&mut self.previous_keys, &mut self.current_keys);
        self.current_keys = self.previous_keys.clone();
        std::mem::swap(&mut self.previous_mouse, &mut self.current_mouse);
        self.current_mouse = self.previous_mouse.clone();
        self.mouse_delta = (0.0, 0.0);
    }

    /// Records a key press or release. Called by the engine from event polling.
    pub fn set_key(&mut self, scancode: Scancode, is_pressed: bool) {
        if is_pressed {
            self.current_keys.insert(scancode);
        } else {
            self.current_keys.remove(&scancode);
        }
    }

    /// Returns `true` if the key is currently held down.
    pub fn is_key_down(&self, scancode: Scancode) -> bool {
        self.current_keys.contains(&scancode)
    }

    /// Returns `true` if the key was pressed this frame (edge-triggered).
    pub fn is_key_pressed(&self, scancode: Scancode) -> bool {
        self.current_keys.contains(&scancode) && !self.previous_keys.contains(&scancode)
    }

    /// Records a mouse button press or release. Called by the engine from event polling.
    pub fn set_mouse_button(&mut self, button: MouseButton, is_pressed: bool) {
        if is_pressed {
            self.current_mouse.insert(button);
        } else {
            self.current_mouse.remove(&button);
        }
    }

    /// Returns `true` if the mouse button is currently held down.
    pub fn is_mouse_down(&self, button: MouseButton) -> bool {
        self.current_mouse.contains(&button)
    }

    /// Returns `true` if the mouse button was pressed this frame (edge-triggered).
    pub fn is_mouse_pressed(&self, button: MouseButton) -> bool {
        self.current_mouse.contains(&button) && !self.previous_mouse.contains(&button)
    }

    /// Accumulates mouse movement for this frame. Called by the engine from event polling.
    pub fn add_mouse_delta(&mut self, x: f32, y: f32) {
        self.mouse_delta.0 += x;
        self.mouse_delta.1 += y;
    }

    /// Returns the accumulated mouse delta `(dx, dy)` for this frame.
    pub fn get_mouse_delta(&self) -> (f32, f32) {self.mouse_delta}
}