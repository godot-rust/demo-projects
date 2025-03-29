use godot::classes::AnimationPlayer;
use godot::classes::CharacterBody3D;
use godot::classes::ICharacterBody3D;
use godot::prelude::*;
use rand::Rng;
use std::f32::consts::PI;

#[derive(GodotClass)]
#[class(init, base=CharacterBody3D)]
pub struct Mob {
    // Minimum speed of the mob in meters per second.
    #[export]
    min_speed: f32,
    // Maximum speed of the mob in meters per second.
    #[export]
    max_speed: f32,
    base: Base<CharacterBody3D>,
}

#[godot_api]
impl ICharacterBody3D for Mob {
    
    fn physics_process(&mut self, _delta: f64) {
        // move_and_slide()
        self.base_mut().move_and_slide();
    }
}
#[godot_api]
impl Mob {
    #[func]
    pub fn initialize(&mut self, start_position: Vector3, player_position: Vector3) {
        // look_at_from_position(start_position, player_position, Vector3.UP)
        self.base_mut()
            .look_at_from_position(start_position, player_position);

        // rotate_y(randf_range(-PI / 4, PI / 4))
        self.base_mut()
            .rotate_y(rand::rng().random_range(-PI / 4.0..PI / 4.0));

        // var random_speed = randf_range(min_speed, max_speed)
        let random_speed = rand::rng().random_range(self.min_speed..self.max_speed);

        // We calculate a forward velocity first, which represents the speed.
        // velocity = Vector3.FORWARD * random_speed
        self.base_mut()
            .set_velocity(Vector3::FORWARD * random_speed);

        let rotation = self.base().get_rotation();
        let velocity = self.base().get_velocity();

        // We then rotate the vector based on the mob's Y rotation to move in the direction it's looking.
        // velocity = velocity.rotated(Vector3.UP, rotation.y)
        self.base_mut()
            .set_velocity(velocity.rotated(Vector3::UP, rotation.y));

        // $AnimationPlayer.speed_scale = random_speed / min_speed
        let animation_speed = rand::rng().random_range(1.0..6.0);
        self.base()
            .get_node_as::<AnimationPlayer>("AnimationPlayer")
            .set_speed_scale(animation_speed as f32)
    }
    // Emitted when the player jumped on the mob.
    // signal squashed
    #[signal]
    pub fn squashed();

    #[func]
    pub fn squash(&mut self) {
        // squashed.emit()
        self.signals().squashed().emit();
        
        // queue_free()
        self.base_mut().queue_free();
    }

    #[func]
    fn on_visible_on_screen_notifier_3d_screen_exited(&mut self) {
        // queue_free()
        self.base_mut().queue_free();
    }
}
