mod main_scene;
mod mob;
mod player;
mod scorelabel;
use godot::prelude::*;

struct SquashTheCreeps;

#[gdextension]
unsafe impl ExtensionLibrary for SquashTheCreeps {}
