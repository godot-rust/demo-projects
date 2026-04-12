use godot::prelude::*;

mod bomb;
mod bomb_spawner;
mod gamestate;
mod lobby;

struct RustExtension;

#[gdextension]
unsafe impl ExtensionLibrary for RustExtension {}
