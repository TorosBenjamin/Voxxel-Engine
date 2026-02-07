# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

VoxxelEngine is a voxel game engine library written in Rust using SDL2 + raw OpenGL 4.5. It is structured as a library crate (`src/lib.rs`) — there is no `main.rs`. Game projects depend on this crate and implement the `VoxxelGame` trait.

## Build & Test Commands

```bash
cargo build              # Build the library
cargo test               # Run all tests
cargo test frustum       # Run a specific test by name
cargo clippy             # Lint
```

**System dependency:** SDL2 development libraries must be installed (`libsdl2-dev` on Ubuntu/Debian).

## Architecture

### Engine Loop (`engine::engine`)

`VoxxelEngine::run()` owns the main loop: poll SDL2 events → update → render → swap. Games implement the `VoxxelGame` trait (`game.rs`) with an associated `Resources` type.

The render flow: `game.render(&mut ctx)` (game submits to queues) → `renderer.render(&mut ctx, game.resources())` (engine processes queues). The game never touches the `Renderer` directly — it only submits `RenderCommand`s.

### Render Pipeline (`render/`)

Three-queue rendering through `RenderContext`:
- `opaque_queue`, `transparent_queue`, `gui_queue` — each is a `RenderQueue` of `RenderCommand`s
- `RenderCommand` references resources via `Handle<GpuMesh>` and `Handle<Material>`, plus optional per-draw `Uniform`s
- `Renderer` (crate-internal) sorts by material, resolves handles via `ResourceAccess`, tracks GL state to skip redundant binds
- GUI queue: Renderer disables depth test, enables blending, uses orthographic projection (`gui_projection`)
- `GuiContext` still exists as a separate immediate-mode path for `Font`/`GuiMaterial` (legacy, not yet unified)

### Resource System (`resource/`, `core/`)

Type-erased asset storage using `TypeId`-keyed `HashMap`:
- `Handle<T>` is a lightweight typed ID (u32 + PhantomData), `Copy` for all `T` (manual impls, no `T: Copy` bound)
- `ResourceManager::insert<T>(value) -> Handle<T>` — stores programmatically-built resources
- `ResourceManager::load<A: Asset>(path, file) -> Handle<A>` — loads from file via `Asset` trait
- `ResourceAccess` trait with `get<T: 'static>(handle) -> Option<&T>` — bound is `'static`, not `Asset`

### File System (`files/`)

Virtual file system with three mount points (`Engine`, `Game`, `User`):
- `FileManager` resolves `LogicalPath` enums to physical paths
- Games define their own `LogicalPath` enum mapping to mount + relative path + `DirPolicy` (Required/AutoCreate/Optional)

### Graphics (`graphics/`)

- `Shader` — compiles GLSL vertex+fragment, provides uniform setters (`set_mat4`, `set_vec3`, etc.)
- `GpuMesh` — uploads vertices to VAO/VBO, supports custom vertex layouts via the `Vertex` trait
- `Texture` / `TextureArray` / `TextureAtlas` — 2D textures, array textures for voxel blocks, atlas UV calculation
- `Font` — TTF rasterization via fontdue into a grayscale texture atlas
- `Material` holds `Handle<Shader>` + `Vec<TextureSlot>` with `TextureBinding` enum (Texture2D or Array). Builder: `Material::new(shader).with_texture(slot, name, binding)`
- `GuiMaterial` holds owned `Shader` + `Texture` (legacy immediate-mode path, not through handle system)
- Built-in vertex types: `VertexPosUv` (pos3+uv2), `VertexPosNormalUv` (pos3+normal3+uv2)

### Input (`input/`)

- `Input` — tracks current/previous key+mouse state per frame; supports `is_key_down`/`is_key_pressed` (edge detection)
- `ActionMapper<A>` — maps a game-defined action enum to multiple `InputSource`s (keyboard or mouse)

### Physics (`physics/`)

AABB-based axis-by-axis collision with gravity and friction:
- `PhysicsSystem::step()` applies gravity, drag, and moves along each axis independently
- `PhysicsEntity` — position, velocity, size (AABB), grounded flag
- `KinematicBody` trait — games implement this to expose their physics entity
- `CollisionMap` trait — games implement `is_solid_at` and `raycast` for world queries
- `Coordinates` — integer block position with `neighbors()` helper

### Math (`math/`)

- `Frustum` — Gribb-Hartmann plane extraction from VP matrix, `intersects_aabb` for culling

## Key Conventions

- `nalgebra_glm` is aliased as `glm` throughout the codebase
- OpenGL calls use `unsafe` blocks with the `gl` crate (raw function pointers)
- GPU resources (`GpuMesh`, `Shader`, `Texture`) implement `Drop` for cleanup
- Shaders are loaded from `assets/shaders/` at runtime (not embedded)
- The engine targets OpenGL 4.5 Core Profile
