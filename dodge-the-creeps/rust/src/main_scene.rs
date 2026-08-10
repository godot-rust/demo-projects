/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::{hud, mob, player};

use godot::classes::{AudioStreamPlayer, Marker2D, PathFollow2D, Timer};
use godot::prelude::*;

use std::f32::consts::PI;

// Deriving GodotClass makes the class available to Godot.
#[derive(GodotClass)]
#[class(base=Node)]
pub struct Main {
    mob_scene: OnReady<Gd<PackedScene>>,
    player: OnReady<Gd<player::Player>>,
    hud: OnReady<Gd<hud::Hud>>,
    music: OnReady<Gd<AudioStreamPlayer>>,
    death_sound: OnReady<Gd<AudioStreamPlayer>>,
    score: i64,
    base: Base<Node>,
}

#[godot_api]
impl INode for Main {
    fn init(base: Base<Node>) -> Self {
        // We could also initialize those manually inside ready(), but OnReady automatically defers initialization.
        // Alternatively to init(), you can use #[init(...)] on the struct fields.
        Self {
            // OnReady::from_loaded(path) == OnReady::new(|| tools::load(path)).
            mob_scene: OnReady::from_loaded("res://Mob.tscn"),
            player: OnReady::from_node("Player"),
            hud: OnReady::from_node("Hud"),
            music: OnReady::from_node("Music"),
            death_sound: OnReady::from_node("DeathSound"),
            score: 0,
            base,
        }
    }

    fn ready(&mut self) {
        // The OnReady instances are now initialized, we can access them like normal fields.

        // Get a Gd<Main> pointer to this instance.
        let main = self.to_gd();

        // Connect Player::hit -> Main::game_over.
        self.player
            .signals()
            .hit()
            .connect_other(&main, Self::game_over);

        // Connect Hud::start_game -> Main::new_game.
        self.hud
            .signals()
            .start_game()
            .connect_other(&main, Self::new_game);

        // Connect Main.ScoreTimer::timeout -> Main::on_score_timer_timeout.
        self.score_timer()
            .signals()
            .timeout()
            .connect_other(&main, Self::on_score_timer_timeout);

        // Connect Main.MobTimer::timeout -> Main::on_mob_timer_timeout.
        self.mob_timer()
            .signals()
            .timeout()
            .connect_other(&main, Self::on_mob_timer_timeout);

        // Main.StartTimer::timeout -> Main::on_start_timer_timeout is set up in the Editor's Inspector UI, but could be done here as well,
        // as follows. Note that signal handlers connected via Rust do not need a #[func] annotation, they can remain entirely visible to Godot.
        //
        // self.start_timer()
        //     .signals()
        //     .timeout()
        //     .connect_other(&main, Self::on_start_timer_timeout);
    }
}

#[godot_api]
impl Main {
    // No #[func] here, this method is directly called from Rust (via type-safe signals).
    fn game_over(&mut self) {
        self.score_timer().stop();
        self.mob_timer().stop();

        self.hud.bind_mut().show_game_over();

        self.music.stop();
        self.death_sound.play();
    }

    // No #[func].
    pub fn new_game(&mut self) {
        let start_position = self.base().get_node_as::<Marker2D>("StartPosition");

        self.score = 0;

        self.player.bind_mut().start(start_position.get_position());
        self.start_timer().start();

        let hud = self.hud.bind_mut();
        hud.update_score(self.score);
        hud.show_message("Get Ready".into());

        self.music.play();
    }

    #[func] // needed because connected in Editor UI (see ready).
    fn on_start_timer_timeout(&mut self) {
        self.mob_timer().start();
        self.score_timer().start();
    }

    // No #[func], connected in pure Rust.
    fn on_score_timer_timeout(&mut self) {
        self.score += 1;

        self.hud.bind_mut().update_score(self.score);
    }

    // No #[func], connected in pure Rust.
    fn on_mob_timer_timeout(&mut self) {
        let mut mob_spawn_location = self
            .base()
            .get_node_as::<PathFollow2D>("MobPath/MobSpawnLocation");

        // Engine methods such as set_position() are available through Gd<Mob>, so there is no need to
        // instantiate as RigidBody2D and cast afterwards.
        let mut mob = self.mob_scene.instantiate_as::<mob::Mob>();

        let progress = rand::random::<u32>();

        mob_spawn_location.set_progress(progress as f32);
        mob.set_position(mob_spawn_location.get_position());

        let mut direction = mob_spawn_location.get_rotation() + PI / 2.0;
        direction += rand::random_range(-PI / 4.0..PI / 4.0);

        mob.set_rotation(direction);

        let range = {
            // Local scope to bind `mob` user object
            let mob = mob.bind();
            rand::random_range(mob.min_speed..mob.max_speed)
        };

        mob.set_linear_velocity(Vector2::new(range, 0.0).rotated(real::from_f32(direction)));

        // Connect Hud::start_game -> Mob::on_start_game, so that mobs from the previous round are
        // removed when a new game begins.
        self.hud
            .signals()
            .start_game()
            .connect_other(&mob, mob::Mob::on_start_game);

        self.base_mut().add_child(&mob);
    }

    // These timers could also be stored as OnReady fields, but are now fetched via function for demonstration purposes.
    fn start_timer(&self) -> Gd<Timer> {
        self.base().get_node_as::<Timer>("StartTimer")
    }

    fn score_timer(&self) -> Gd<Timer> {
        self.base().get_node_as::<Timer>("ScoreTimer")
    }

    fn mob_timer(&self) -> Gd<Timer> {
        self.base().get_node_as::<Timer>("MobTimer")
    }
}
