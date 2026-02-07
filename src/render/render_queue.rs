use crate::render::render_command::RenderCommand;

/// An ordered list of render commands processed by the renderer.
pub struct RenderQueue {
    commands: Vec<RenderCommand>,
}

impl RenderQueue {
    /// Creates an empty render queue.
    pub fn new() -> Self {
        Self { commands: Vec::new() }
    }

    /// Adds a render command to the queue.
    pub fn submit(&mut self, cmd: RenderCommand) {
        self.commands.push(cmd);
    }

    /// Removes all commands from the queue.
    pub fn clear(&mut self) {
        self.commands.clear();
    }

    /// Sorts commands by material handle to minimize GPU state changes.
    pub fn sort_by_material(&mut self) {
        self.commands.sort_by_key(|cmd| cmd.material.id);
    }

    /// Returns an iterator over the queued commands.
    pub fn iter(&self) -> std::slice::Iter<'_, RenderCommand> {
        self.commands.iter()
    }
}

impl<'a> IntoIterator for &'a RenderQueue {
    type Item = &'a RenderCommand;
    type IntoIter = std::slice::Iter<'a, RenderCommand>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

