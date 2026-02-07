use crate::engine::builtins::BuiltinResources;
use crate::engine::context::EngineContext;
use crate::engine::gui_context::GuiContext;
use crate::render::render_context::RenderContext;
use crate::resource::resource_manager::ResourceStore;

/// Trait implemented by games that run on the engine.
pub trait VoxxelGame {
    /// The game's resource storage, used by the renderer to resolve handles.
    type Resources: ResourceStore;

    /// Called once after the OpenGL context is ready, with handles to built-in resources.
    fn on_init(&mut self, builtins: BuiltinResources);
    /// Called once per frame to update game logic.
    fn update(&mut self, ctx: &mut EngineContext);
    /// Called once per frame to submit render commands to the queues.
    fn render(&mut self, ctx: &mut RenderContext);
    /// Called once per frame to draw immediate-mode GUI elements.
    fn render_ui(&self, ctx: &GuiContext);
    /// Returns a reference to the game's resource storage.
    fn resources(&self) -> &Self::Resources;
    /// Returns a mutable reference to the game's resource storage.
    fn resources_mut(&mut self) -> &mut Self::Resources;
}
