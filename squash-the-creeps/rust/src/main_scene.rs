/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::mob::Mob;
use crate::player::Player;
use crate::user_interface::UserInterface;

use godot::classes::{ColorRect, InputEvent, PathFollow3D, Timer};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct MainScene {
    #[export]
    mob_scene: OnEditor<Gd<PackedScene>>,

    #[init(node = "MobTimer")]
    mob_timer: OnReady<Gd<Timer>>,

    #[init(node = "Player")]
    player: OnReady<Gd<Player>>,

    #[init(node = "UserInterface")]
    user_interface: OnReady<Gd<UserInterface>>,

    #[init(node = "UserInterface/Retry")]
    retry: OnReady<Gd<ColorRect>>,

    base: Base<Node>,
}

#[godot_api]
impl INode for MainScene {
    fn ready(&mut self) {
        self.retry.hide();

        self.player
            .signals()
            .hit()
            .connect_other(&self.to_gd(), Self::on_player_hit);
    }

    fn unhandled_input(&mut self, event: Gd<InputEvent>) {
        if event.is_action_pressed("ui_accept") && self.retry.is_visible() {
            self.base().get_tree().reload_current_scene();
        }
    }
}

#[godot_api]
impl MainScene {
    #[func] // needed because connected in Editor UI (main.tscn).
    fn on_mob_timer_timeout(&mut self) {
        // Create mob instance.
        let mut mob_spawn_location = self
            .base()
            .get_node_as::<PathFollow3D>("SpawnPath/SpawnLocation");

        // Choose a random location on the SpawnPath.
        // Set random progress using proper rng.
        mob_spawn_location.set_progress_ratio(rand::random_range(0.0..=1.0));

        // Communicate the spawn location and the player's location to the mob.
        let player_position = self.player.get_position();

        let mut mob = self.mob_scene.instantiate_as::<Mob>();

        mob.bind_mut()
            .initialize(mob_spawn_location.get_position(), player_position);

        // Spawn the mob by adding it to the Main scene.
        self.base_mut().add_child(&mob);

        // We connect the mob to the score label to update the score upon squashing a mob.
        mob.signals()
            .squashed()
            .connect_other(&*self.user_interface, UserInterface::on_mob_squashed);
    }

    // No #[func], connected in pure Rust.
    fn on_player_hit(&mut self) {
        self.mob_timer.stop();
        self.retry.show();
    }
}
