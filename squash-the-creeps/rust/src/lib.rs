mod main_scene;
mod mob;
mod player;
mod scorelabel;
use godot::prelude::*;

struct Scripts;

#[gdextension]
unsafe impl ExtensionLibrary for Scripts {}
