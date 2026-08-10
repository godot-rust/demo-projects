/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use godot::classes::{Button, CanvasLayer, ICanvasLayer, Label, Timer};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=CanvasLayer)]
pub struct Hud {
    base: Base<CanvasLayer>,
}

#[godot_api]
impl Hud {
    // Public signal, since it's used by Main struct.
    #[signal]
    pub fn start_game();

    // No #[func], called directly from Rust.
    pub fn show_message(&self, text: GString) {
        let mut message_label = self.base().get_node_as::<Label>("MessageLabel");
        message_label.set_text(&text);
        message_label.show();

        let mut timer = self.base().get_node_as::<Timer>("MessageTimer");
        timer.start();
    }

    pub fn show_game_over(&self) {
        self.show_message("Game Over".into());

        // Engine classes have type-safe signals, too.
        let timer = self.base().get_tree().create_timer(2.0);
        timer
            .signals()
            .timeout()
            .connect_other(&self.to_gd(), Self::show_start_button);
    }

    // No #[func], connected in pure Rust.
    fn show_start_button(&mut self) {
        let mut message_label = self.base().get_node_as::<Label>("MessageLabel");
        message_label.set_text("Dodge the\nCreeps!");
        message_label.show();

        let mut button = self.base().get_node_as::<Button>("StartButton");
        button.show();
    }

    // No #[func], called directly from Rust.
    pub fn update_score(&self, score: i64) {
        let mut label = self.base().get_node_as::<Label>("ScoreLabel");

        label.set_text(&score.to_string());
    }

    #[func]
    fn on_start_button_pressed(&mut self) {
        let mut button = self.base().get_node_as::<Button>("StartButton");
        button.hide();

        self.signals().start_game().emit();
    }

    #[func]
    fn on_message_timer_timeout(&self) {
        let mut message_label = self.base().get_node_as::<Label>("MessageLabel");
        message_label.hide()
    }
}

#[godot_api]
impl ICanvasLayer for Hud {
    fn init(base: Base<Self::Base>) -> Self {
        Self { base }
    }
}
