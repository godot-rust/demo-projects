/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use godot::classes::{AnimatedSprite2D, Area2D, CollisionShape2D, IArea2D, Input};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct Player {
    speed: real,
    screen_size: Vector2,

    base: Base<Area2D>,
}

#[godot_api]
impl Player {
    // Public signal, since it's used by Main struct.
    #[signal]
    pub fn hit();

    // No #[func], connected in pure Rust (see ready).
    fn on_player_body_entered(&mut self, _body: Gd<Node2D>) {
        self.base_mut().hide();
        self.signals().hit().emit();

        let mut collision_shape = self
            .base()
            .get_node_as::<CollisionShape2D>("CollisionShape2D");

        // Disabling a collision shape must be deferred, as we are inside a physics callback.
        // Godot exposes no typed API for deferred property writes, hence the string-based one.
        collision_shape.set_deferred("disabled", &true.to_variant());
    }

    #[func]
    pub fn start(&mut self, pos: Vector2) {
        self.base_mut().set_global_position(pos);
        self.base_mut().show();

        let mut collision_shape = self
            .base()
            .get_node_as::<CollisionShape2D>("CollisionShape2D");

        collision_shape.set_disabled(false);
    }
}

#[godot_api]
impl IArea2D for Player {
    fn init(base: Base<Area2D>) -> Self {
        Player {
            speed: 400.0,
            screen_size: Vector2::new(0.0, 0.0),
            base,
        }
    }

    fn ready(&mut self) {
        let viewport = self.base().get_viewport_rect();
        self.screen_size = viewport.size;
        self.base_mut().hide();

        // Signal setup
        self.signals()
            .body_entered()
            .connect_self(Self::on_player_body_entered);
    }

    // `delta` can be f32 or f64; #[godot_api] macro converts transparently.
    fn process(&mut self, delta: f32) {
        let mut animated_sprite = self
            .base()
            .get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");

        let mut velocity = Vector2::new(0.0, 0.0);

        let input = Input::singleton();
        if input.is_action_pressed("move_right") {
            velocity += Vector2::RIGHT;
        }
        if input.is_action_pressed("move_left") {
            velocity += Vector2::LEFT;
        }
        if input.is_action_pressed("move_down") {
            velocity += Vector2::DOWN;
        }
        if input.is_action_pressed("move_up") {
            velocity += Vector2::UP;
        }

        if velocity.length() > 0.0 {
            velocity = velocity.normalized() * self.speed;

            let animation = if velocity.x != 0.0 {
                animated_sprite.set_flip_v(false);
                animated_sprite.set_flip_h(velocity.x < 0.0);
                "right"
            } else {
                animated_sprite.set_flip_v(velocity.y > 0.0);
                "up"
            };

            animated_sprite.play_ex().name(animation).done();
        } else {
            animated_sprite.stop();
        }

        let change = velocity * delta;
        let position = self.base().get_global_position() + change;
        let position = Vector2::new(
            position.x.clamp(0.0, self.screen_size.x),
            position.y.clamp(0.0, self.screen_size.y),
        );
        self.base_mut().set_global_position(position);
    }
}
