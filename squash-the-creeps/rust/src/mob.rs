/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use godot::classes::{AnimationPlayer, CharacterBody3D, ICharacterBody3D};
use godot::prelude::*;
use std::f32::consts::PI;

#[derive(GodotClass)]
#[class(init, base=CharacterBody3D)]
pub struct Mob {
    /// Minimum speed of the mob in meters per second.
    #[export]
    #[init(val = 10.0)]
    min_speed: f32,

    /// Maximum speed of the mob in meters per second.
    #[export]
    #[init(val = 18.0)]
    max_speed: f32,

    base: Base<CharacterBody3D>,
}

#[godot_api]
impl ICharacterBody3D for Mob {
    fn physics_process(&mut self, _delta: f64) {
        self.base_mut().move_and_slide();
    }
}

#[godot_api]
impl Mob {
    // No #[func], called directly from Rust (see MainScene::on_mob_timer_timeout).
    pub fn initialize(&mut self, start_position: Vector3, player_position: Vector3) {
        self.base_mut()
            .look_at_from_position(start_position, player_position);

        self.base_mut()
            .rotate_y(rand::random_range(-PI / 4.0..PI / 4.0));

        let random_speed = rand::random_range(self.min_speed..self.max_speed);

        // Compute the forward velocity, then rotate it by the mob's Y rotation, so that it moves in
        // the direction it's looking.
        let rotation_y = self.base().get_rotation().y;
        let velocity = (Vector3::FORWARD * random_speed).rotated(Vector3::UP, rotation_y);
        self.base_mut().set_velocity(velocity);

        let animation_speed = rand::random_range(1.0_f32..6.0);

        self.base()
            .get_node_as::<AnimationPlayer>("AnimationPlayer")
            .set_speed_scale(animation_speed);
    }

    // Emitted when the player jumped on the mob.
    #[signal]
    pub fn squashed();

    // No #[func], called directly from Rust (see Player::physics_process).
    pub fn squash(&mut self) {
        self.signals().squashed().emit();

        self.base_mut().queue_free();
    }

    #[func]
    fn on_visible_on_screen_notifier_3d_screen_exited(&mut self) {
        self.base_mut().queue_free();
    }
}
