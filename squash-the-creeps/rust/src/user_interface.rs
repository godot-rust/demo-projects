/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use godot::classes::{Control, Label};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Control)]
pub struct UserInterface {
    score: u32,
    base: Base<Control>,
}

#[godot_api]
impl UserInterface {
    // No #[func], connected in pure Rust (see MainScene::on_mob_timer_timeout).
    pub fn on_mob_squashed(&mut self) {
        self.score += 1;

        let mut label = self.base().get_node_as::<Label>("ScoreLabel");
        label.set_text(format!("Score: {}", self.score).as_str());
    }
}
