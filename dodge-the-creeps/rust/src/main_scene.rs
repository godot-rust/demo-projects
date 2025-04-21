use crate::{hud, mob, player};

use godot::classes::{AudioStreamPlayer, Marker2D, PathFollow2D, RigidBody2D, Timer};
use godot::prelude::*;

use rand::Rng as _;
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
            .connect_obj(&main, Self::game_over);

        // Connect Hud::start_game -> Main::new_game.
        self.hud
            .signals()
            .start_game()
            .connect_obj(&main, Self::new_game);

        // Connect Main.ScoreTimer::timeout -> Main::on_score_timer_timeout.
        self.score_timer()
            .signals()
            .timeout()
            .connect_obj(&main, Self::on_score_timer_timeout);

        // Connect Main.MobTimer::timeout -> Main::on_mob_timer_timeout.
        self.mob_timer()
            .signals()
            .timeout()
            .connect_obj(&main, Self::on_mob_timer_timeout);

        // Main.StartTimer::timeout -> Main::on_start_timer_timeout is set up in the Editor's Inspector UI, but could be done here as well,
        // as follows. Note that signal handlers connected via Rust do not need a #[func] annotation, they can remain entirely visible to Godot.
        //
        // self.start_timer()
        //     .signals()
        //     .timeout()
        //     .connect_obj(&main, Self::on_start_timer_timeout);
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

        let mut mob_scene = self.mob_scene.instantiate_as::<RigidBody2D>();

        let mut rng = rand::thread_rng();
        let progress = rng.gen_range(u32::MIN..u32::MAX);

        mob_spawn_location.set_progress(progress as f32);
        mob_scene.set_position(mob_spawn_location.get_position());

        let mut direction = mob_spawn_location.get_rotation() + PI / 2.0;
        direction += rng.gen_range(-PI / 4.0..PI / 4.0);

        mob_scene.set_rotation(direction);

        self.base_mut().add_child(&mob_scene);

        let mut mob = mob_scene.cast::<mob::Mob>();
        let range = {
            // Local scope to bind `mob` user object
            let mob = mob.bind();
            rng.gen_range(mob.min_speed..mob.max_speed)
        };

        mob.set_linear_velocity(Vector2::new(range, 0.0).rotated(real::from_f32(direction)));
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
