use godot::prelude::*;

struct RustExtension;

#[gdextension]
unsafe impl ExtensionLibrary for RustExtension {}

mod ball;
mod lobby;
mod paddle;
mod pong;
