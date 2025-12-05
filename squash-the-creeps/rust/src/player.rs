use crate::mob::Mob;
use godot::classes::{AnimationPlayer, CharacterBody3D, CollisionShape3D, ICharacterBody3D, Input};
use godot::prelude::*;
use std::f32::consts::FRAC_PI_6;

#[derive(GodotClass)]
#[class(init, base=CharacterBody3D)]
pub struct Player {
    /// How fast the player moves in meters per second.
    #[export]
    speed: f32,

    /// Vertical impulse applied to the character upon jumping in meters per second.
    #[export]
    jump_impulse: f32,

    /// Vertical impulse applied to the character upon bouncing over a mob in meters per second.
    #[export]
    bounce_impulse: f32,

    /// The downward acceleration when in the air, in meters per second.
    #[export]
    fall_acceleration: f32,

    /// The target velocity of the character (node property)
    #[export]
    target_velocity: Vector3,
    base: Base<CharacterBody3D>,
}
#[godot_api]
impl ICharacterBody3D for Player {
    fn physics_process(&mut self, delta: f64) {
        let mut direction = Vector3::ZERO;

        let input = Input::singleton();

        if input.is_action_pressed("move_right") {
            direction += Vector3::RIGHT;
        }
        if input.is_action_pressed("move_left") {
            direction += Vector3::LEFT;
        }
        if input.is_action_pressed("move_back") {
            direction += Vector3::BACK;
        }
        if input.is_action_pressed("move_forward") {
            direction += Vector3::FORWARD;
        }

        if direction != Vector3::ZERO {
            // In the lines below, we turn the character when moving and make the animation play faster.
            direction = direction.normalized();

            // Take the pivot node and rotate it to face the direction we're moving in.
            let mut pivot = self.base_mut().get_node_as::<Node3D>("Pivot");

            // Setting the basis property will affect the rotation of the node.
            pivot.set_basis(Basis::looking_at(-direction));
            self.base()
                .get_node_as::<AnimationPlayer>("AnimationPlayer")
                .set_speed_scale(4.0);
        } else {
            self.base()
                .get_node_as::<AnimationPlayer>("AnimationPlayer")
                .set_speed_scale(1.0);
        }
        // Ground Velocity.
        self.target_velocity.x = direction.x * self.speed;
        self.target_velocity.z = direction.z * self.speed;

        // jumping.
        if self.base().is_on_floor() && input.is_action_just_pressed("jump") {
            self.target_velocity.y = self.jump_impulse;
        }
        // We apply gravity every frame so the character always collides with the ground when moving.
        // This is necessary for the is_on_floor() function to work as a body can always detect
        // the floor, walls, etc. when a collision happens the same frame.
        if !self.base().is_on_floor() {
            self.target_velocity.y -= self.fall_acceleration * delta as f32;
        }
        // moving the Character
        let velocity = self.target_velocity;
        self.base_mut().set_velocity(velocity);
        self.base_mut().move_and_slide();

        // Here, we check if we landed on top of a mob and if so, we kill it and bounce.
        // With move_and_slide(), Godot makes the body move sometimes multiple times in a row to
        // smooth out the character's motion. So we have to loop over all collisions that may have
        // happened.
        // If there are no "slides" this frame, the loop below won't run.
        for index in 0..self.base().get_slide_collision_count() {
            let collision = self.base_mut().get_slide_collision(index).unwrap();
            // Skip given collider if they are not a Mob.
            // Mob is an instance of Mob class in the "mob" group.
            let Some(Ok(mut mob)) = collision
                .get_collider()
                .map(Gd::cast::<Node3D>)
                .filter(|n| n.is_in_group("mob"))
                .map(|n| n.try_cast::<Mob>())
            else {
                continue;
            };
            if Vector3::UP.dot(collision.get_normal()) > 0.1 {
                mob.bind_mut().squash();
                self.target_velocity.y = self.bounce_impulse;
                // Prevent this block from running more than once,
                // which would award the player more than 1 point for squashing a single mob.
                break;
            }
        }
        // This makes the character follow a nice arc when jumping.
        let mut pivot = self.base().get_node_as::<Node3D>("Pivot");
        let mut pivot_rotation = pivot.get_rotation();
        pivot_rotation.x = FRAC_PI_6 * self.base().get_velocity().y / self.jump_impulse;
        pivot.set_rotation(pivot_rotation);
    }
}

#[godot_api]
impl Player {
    #[signal]
    pub fn hit();

    #[func]
    pub fn die(&mut self) {
        self.signals().hit().emit();
        self.base_mut().queue_free();
    }

    #[func]
    pub fn on_mob_detector_body_entered(&mut self, _body: Gd<CharacterBody3D>) {
        let mut collision_shape = self
            .base()
            .get_node_as::<CollisionShape3D>("CollisionShape3D");
        collision_shape.set_deferred("disabled", &true.to_variant());

        self.die();
    }
}
