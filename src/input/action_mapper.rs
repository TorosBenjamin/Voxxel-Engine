use std::collections::HashMap;
use std::hash::Hash;
use crate::input::input::Input;
use crate::input::input_source::InputSource;

/// Maps game-defined action enums to physical inputs and tracks their state.
pub struct ActionMapper<A: Eq + Hash + Clone> {
    bindings: HashMap<A, Vec<InputSource>>,
    active_states: HashMap<A, bool>,
    pressed_states: HashMap<A, bool>,
}

impl<A: Eq + Hash + Clone> ActionMapper<A> {
    /// Creates an empty action mapper with no bindings.
    pub fn new() -> Self {Self{
        bindings: HashMap::new(),
        active_states: HashMap::new(),
        pressed_states: HashMap::new(),
    }}
    /// Reads current input state and updates all action states. Call once per frame.
    pub fn update(&mut self, input: &Input) {
        for (action, sources) in &self.bindings {
            let is_down = sources.iter().any(|s| match s {
                InputSource::Key(k) => input.is_key_down(*k),
                InputSource::Mouse(m) => input.is_mouse_down(*m),
            });

            let is_pressed = sources.iter().any(|s| match s {
                InputSource::Key(k) => input.is_key_pressed(*k),
                InputSource::Mouse(m) => input.is_mouse_pressed(*m),
            });

            self.active_states.insert(action.clone(), is_down);
            self.pressed_states.insert(action.clone(), is_pressed);
        }
    }

    /// Returns `true` if any input bound to the action is currently held down.
    pub fn is_active(&self, action: &A) -> bool {
        *self.active_states.get(action).unwrap_or(&false)
    }

    /// Returns `true` if any input bound to the action was pressed this frame.
    pub fn is_pressed(&self, action: &A) -> bool {
        *self.pressed_states.get(action).unwrap_or(&false)
    }

    /// Binds a physical input source to an action. Multiple sources can be bound to one action.
    pub fn bind(&mut self, action: A, source: InputSource) {
        self.bindings
            .entry(action)
            .or_insert_with(Vec::new)
            .push(source);
    }
}