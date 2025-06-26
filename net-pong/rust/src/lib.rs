use godot::prelude::*;

struct RustExtension;

#[gdextension]
unsafe impl ExtensionLibrary for RustExtension {}

fn variant_array_to_vec(array: VariantArray) -> Vec<Variant> {
    let mut vec = Vec::new();
    for i in 0..array.len() {
        vec.push(
            array
                .get(i)
                .expect("Failed to get element from VariantArray"),
        );
    }
    vec
}

mod ball;
mod lobby;
mod paddle;
mod pong;
