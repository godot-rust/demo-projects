use crate::mob;
use crate::player;
use crate::player::Player;
use crate::scorelabel::UserInterface;

use godot::classes::{ColorRect, InputEvent, PathFollow3D, Timer};
use godot::prelude::*;
use rand::Rng;

#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct MainScene {
    #[export]
    mob_scene: OnEditor<Gd<PackedScene>>,

    #[init(node = "MobTimer")]
    mob_timer: OnReady<Gd<Timer>>,

    #[init(node = "UserInterface")]
    user_interface: OnReady<Gd<UserInterface>>,

    base: Base<Node>,
}

#[godot_api]
impl INode for MainScene {
    fn ready(&mut self) {
        self.base()
            .get_node_as::<ColorRect>("UserInterface/Retry")
            .hide();

        self.base()
            .get_node_as::<Player>("Player")
            .signals()
            .hit()
            .connect_other(&self.to_gd(), Self::on_player_hit);
    }

    fn unhandled_input(&mut self, event: Gd<InputEvent>) {
        if event.is_action_pressed("ui_accept")
            && self
                .base()
                .get_node_as::<ColorRect>("UserInterface/Retry")
                .is_visible()
        {
            self.base().get_tree().unwrap().reload_current_scene();
        }
    }
}

#[godot_api]
impl MainScene {
    #[func]
    fn on_mob_timer_timeout(&mut self) {
        // Create mob instance.
        let mut mob_spawn_location = self
            .base()
            .get_node_as::<PathFollow3D>("SpawnPath/SpawnLocation");

        // Choose a random location on the SpawnPath.
        // Set random progress using proper rng.
        mob_spawn_location.set_progress_ratio(rand::rng().random_range(0.0..=1.0));

        // Communicate the spawn location and the player's location to the mob.
        let player_position = self
            .base()
            .get_node_as::<player::Player>("Player")
            .get_position();

        let mut mob = self.mob_scene.instantiate_as::<mob::Mob>();

        mob.bind_mut()
            .initialize(mob_spawn_location.get_position(), player_position);

        // Spawn the mob by adding it to the Main scene.
        self.base_mut().add_child(&mob);

        // We connect the mob to the score label to update the score upon squashing a mob.
        mob.signals()
            .squashed()
            .connect_other(&*self.user_interface, UserInterface::on_mob_squashed);
    }

    #[func]
    pub fn on_player_hit(&mut self) {
        self.mob_timer.stop();

        self.base()
            .get_node_as::<ColorRect>("UserInterface/Retry")
            .show();
    }
}
