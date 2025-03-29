use crate::mob;
use crate::player;
use crate::scorelabel;

use godot::classes::ColorRect;
use godot::classes::InputEvent;
use godot::classes::PathFollow3D;
use godot::classes::Timer;
use godot::prelude::*;
use rand::Rng;

// Deriving GodotClass makes the class available to Godot.
// extends Node
#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct MainScene {
    // export(PackedScene) var mob_scene
    #[export]
    mob_scene: OnEditor<Gd<PackedScene>>,

    #[init(node = "Player")]
    player: OnReady<Gd<player::Player>>,

    #[init(node = "MobTimer")]
    mob_timer: OnReady<Gd<Timer>>,

    #[init(node = "UserInterface")]
    user_interface: OnReady<Gd<scorelabel::UserInterface>>,

    base: Base<Node>,
}

#[godot_api]
impl INode for MainScene {

    fn ready(&mut self) {
        // $UserInterface/Retry.hide()
        self.base()
            .get_node_as::<ColorRect>("UserInterface/Retry")
            .hide();
    }
    fn unhandled_input(&mut self, event: Gd<InputEvent>) {
        // if event.is_action_pressed("ui_accept") and $UserInterface/Retry.visible:
        if event.is_action_pressed("ui_accept")
            && self
                .base()
                .get_node_as::<ColorRect>("UserInterface/Retry")
                .is_visible()
        {
            // warning-ignore:return_value_discarded
            // get_tree().reload_current_scene()
            self.base().get_tree().unwrap().reload_current_scene();
        }
    }
}
#[godot_api]
impl MainScene {
    #[func]
    fn on_mob_timer_timeout(&mut self) {
        // Create mob instance
        // var mob_spawn_location = get_node("SpawnPath/SpawnLocation")
        let mut mob_spawn_location = self
            .base()
            .get_node_as::<PathFollow3D>("SpawnPath/SpawnLocation");

        // Choose a random location on the SpawnPath.
        // Set random progress using proper rng
        // mob_spawn_location.progress_ratio = randf()
        mob_spawn_location.set_progress_ratio(rand::rng().random_range(0.0..=1.0));
        // Communicate the spawn location and the player's location to the mob.
        //var player_position = $Player.position
        let player_position = self.player.get_position();
        // var mob = mob_scene.instantiate()
        let mut mob = self.mob_scene.instantiate_as::<mob::Mob>();
        // mob.initialize(mob_spawn_location.position, player_position)
        mob.bind_mut()
            .initialize(mob_spawn_location.get_position(), player_position);
        // mob.squashed.connect($UserInterface/ScoreLabel._on_mob_squashed.bind())
        mob.connect(
            "squashed",
            &mut self.user_interface.callable("on_mob_squashed").bind(&[]),
        );

        // add_child(mob)
        self.base_mut().add_child(&mob);
    }

    #[func]
    pub fn on_player_hit(&mut self) {
        // $MobTimer.stop()
        self.mob_timer.stop();
        // $UserInterface/Retry.show()
        self.base()
            .get_node_as::<ColorRect>("UserInterface/Retry")
            .show();
    }
}
