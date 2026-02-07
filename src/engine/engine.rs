use nalgebra_glm as glm;
use sdl2::event::Event;
use crate::camera::Camera;
use crate::engine::builtins::BuiltinResources;
use crate::engine::context::EngineContext;
use crate::engine::gui_context::GuiContext;
use crate::graphics::font::Font;
use crate::graphics::shader::Shader;
use crate::render::render_context::RenderContext;
use crate::render::renderer::Renderer;
use crate::game::VoxxelGame;
use crate::input::input::Input;
use crate::resource::resource_manager::ResourceStore;

/// The main engine that owns the window, input, camera, and render loop.
pub struct VoxxelEngine {
    window: sdl2::video::Window,
    _gl_context: sdl2::video::GLContext,
    event_pump: sdl2::EventPump,
    input: Input,
    renderer: Renderer,
    camera: Camera,
}

impl VoxxelEngine {
    /// Initializes SDL2, creates an OpenGL 4.5 window, and returns a new engine instance.
    pub fn new() -> Self {
        let sdl = sdl2::init().unwrap();
        let video = sdl.video().unwrap();

        let gl_attr = video.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(4, 5);

        let window = video
            .window("Voxxel Engine", 1280, 720)
            .opengl()
            .resizable()
            .build()
            .unwrap();

        let gl_context = window.gl_create_context().unwrap();
        gl::load_with(|s| video.gl_get_proc_address(s) as *const _);

        let event_pump = sdl.event_pump().unwrap();
        sdl.mouse().set_relative_mouse_mode(true);

        // Adaptive VSync: syncs when possible, doesn't stall when behind.
        // Falls back to no VSync if the driver doesn't support it.
        if video.gl_set_swap_interval(sdl2::video::SwapInterval::LateSwapTearing).is_err() {
            let _ = video.gl_set_swap_interval(sdl2::video::SwapInterval::Immediate);
        }

        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::ClearColor(0.5, 0.7, 1.0, 1.0); // Sky blue
        }

        Self {
            window,
            _gl_context: gl_context,
            event_pump,
            input: Input::new(),
            renderer: Renderer::new(),
            camera: Camera::new(glm::vec3(0.0, 0.0, 0.0)),
        }
    }

    /// Returns a reference to the SDL2 window.
    pub fn window(&self) -> &sdl2::video::Window {
        &self.window
    }

    /// Starts the main loop: polls events, updates the game, renders, and swaps buffers.
    pub fn run<G: VoxxelGame>(mut self, mut game: G) {
        // Compile built-in shaders from embedded source
        let voxel_shader = game.resources_mut().insert(Shader::from_source(
            include_str!("../../assets/shaders/vertex.glsl"),
            include_str!("../../assets/shaders/fragment.glsl"),
        ));
        let text_shader = game.resources_mut().insert(Shader::from_source(
            include_str!("../../assets/shaders/text_vertex.glsl"),
            include_str!("../../assets/shaders/text_fragment.glsl"),
        ));
        let ui_shader = game.resources_mut().insert(Shader::from_source(
            include_str!("../../assets/shaders/ui_vertex.glsl"),
            include_str!("../../assets/shaders/ui_fragment.glsl"),
        ));
        let wireframe_shader = game.resources_mut().insert(Shader::from_source(
            include_str!("../../assets/shaders/wireframe_vertex.glsl"),
            include_str!("../../assets/shaders/wireframe_fragment.glsl"),
        ));

        // Rasterize default font from embedded TTF
        let default_font = game.resources_mut().insert(
            Font::from_ttf_bytes(include_bytes!("../../assets/fonts/Pix32.ttf"), 24.0),
        );

        game.on_init(BuiltinResources {
            voxel_shader,
            text_shader,
            ui_shader,
            wireframe_shader,
            default_font,
        });

        let mut last_frame = std::time::Instant::now();

        'running: loop {
            let now = std::time::Instant::now();
            let mut delta_time = now.duration_since(last_frame).as_secs_f32();
            last_frame = now;

            // Prevent huge first-frame delta_time or lag spikes from breaking physics
            if delta_time > 0.1 {
                delta_time = 0.016; // Assume ~60fps if we have a huge lag spike
            }

            while let Some(event) = self.event_pump.poll_event() {
                match event {
                    Event::Quit { .. } => break 'running,
                    Event::KeyDown { scancode: Some(k), .. } => {
                        self.input.set_key(k, true);
                    }
                    Event::KeyUp { scancode: Some(k), .. } => self.input.set_key(k, false),
                    Event::MouseButtonDown { mouse_btn, .. } => { self.input.set_mouse_button(mouse_btn, true); }
                    Event::MouseButtonUp { mouse_btn, .. } => { self.input.set_mouse_button(mouse_btn, false); }
                    Event::MouseMotion { xrel, yrel, .. } => {
                        self.input.add_mouse_delta(xrel as f32, yrel as f32);
                    }
                    _ => {}
                }
            }

            let (w, h) = self.window.size();

            // --- Update ---
            {
                let mut engine_ctx = EngineContext {
                    input: &self.input,
                    delta_time,
                    camera: &mut self.camera,
                    screen_width: w as f32,
                    screen_height: h as f32,
                };

                game.update(&mut engine_ctx);
            }

            // --- Render ---
            unsafe {
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            }

            let aspect = w as f32 / h as f32;
            let mut render_ctx = RenderContext::new(
                self.camera.view_matrix(),
                self.camera.projection_matrix(aspect),
                w as f32,
                h as f32,
            );

            // Game submits commands to queues
            game.render(&mut render_ctx);

            // Engine processes all queues (opaque -> transparent -> gui)
            self.renderer.render(&mut render_ctx, game.resources());

            // GUI immediate-mode path (kept for GuiContext/Font compatibility)
            // Blend is still enabled and depth test disabled from the renderer's GUI pass
            let gui_ctx = GuiContext::new(w as f32, h as f32);
            game.render_ui(&gui_ctx);

            // Restore GL state for next frame
            unsafe {
                gl::Disable(gl::BLEND);
                gl::Enable(gl::DEPTH_TEST);
            }

            self.window.gl_swap_window();

            self.input.update();
        }
    }
}
