/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use godot::prelude::*;

struct HotReload;

#[gdextension]
unsafe impl ExtensionLibrary for HotReload {
    fn on_level_init(level: InitLevel) {
        println!("[Rust]   Init level {level:?}");
    }

    fn on_level_deinit(level: InitLevel) {
        println!("[Rust]   Deinit level {level:?}");
    }
}

// ----------------------------------------------------------------------------------------------------------------------------------------------

/// A RustDoc comment appearing under the editor help docs.
#[derive(GodotClass)]
#[class(init, base=Node)]
struct Reloadable {
    /// A planet!
    #[export]
    #[init(val = Planet::Earth)]
    favorite_planet: Planet,
    //
    // HOT-RELOAD: uncomment this to add a new exported field (also update init() below).
    // #[export]
    // some_string: GString,
}

#[godot_api]
impl Reloadable {
    /// A function to return a number.
    #[func]
    fn get_number(&self) -> i64 {
        // HOT-RELOAD: change returned value for dynamic code change.
        100
    }

    // HOT-RELOAD: uncomment to make new function accessible.

    #[func]
    fn get_planet(&self) -> Planet {
        self.favorite_planet
    }
}

// ----------------------------------------------------------------------------------------------------------------------------------------------

/// A planet enum.
#[derive(GodotConvert, Var, Export, Copy, Clone, Debug)]
#[godot(via = GString)]
enum Planet {
    Earth,
    Mars,
    Venus,
    //
    // HOT-RELOAD: uncomment this to extend enum.
    //Jupiter,
}
