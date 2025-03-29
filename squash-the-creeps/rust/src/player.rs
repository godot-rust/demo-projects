use crate::mob::Mob;
use godot::classes::AnimationPlayer;
use godot::classes::CharacterBody3D;
use godot::classes::CollisionShape3D;
use godot::classes::ICharacterBody3D;
use godot::classes::Input;
use godot::prelude::*;
use std::f32::consts::FRAC_PI_6;

#[derive(GodotClass)]
#[class(base=CharacterBody3D)]
pub struct Player {
    // How fast the player moves in meters per second.
    speed: f32,
    // Vertical impulse applied to the character upon jumping in meters per second.
    jump_impulse: f32,
    // Vertical impulse applied to the character upon bouncing over a mob in meters per second.
    bounce_impulse: f32,
    // The downward acceleration when in the air, in meters per second.
    fall_acceleration: f32,
    // The target velocity of the character (node property)
    target_velocity: Vector3,

    base: Base<CharacterBody3D>,
}
#[godot_api]
impl ICharacterBody3D for Player {
    fn init(base: Base<CharacterBody3D>) -> Self {
        godot_print!("Player initialized");
        Self {
            // @export var speed = 14
            speed: 14.0,
            // @export var jump_impulse = 20
            jump_impulse: 20.0,
            // @export var bounce_impulse = 16
            bounce_impulse: 16.0,
            // @export var fall_acceleration = 75
            fall_acceleration: 75.0,
            // set it to zero
            target_velocity: Vector3::ZERO,
            base,
        }
    }

    fn physics_process(&mut self, _delta: f64) {
        /*Here, instead of _process(), we're going to make all
        calculations using the _physics_process() virtual function.
        It's designed specifically for physics-related code like
        moving a kinematic or rigid body.
        It updates the node using fixed time intervals.*/

        // var direction = Vector3.ZERO
        let mut direction = Vector3::ZERO;

        let input = Input::singleton();
 
        // if Input.is_action_pressed("move_right"):
        if input.is_action_pressed("move_right") {
            //direction.x += 1
            direction += Vector3::RIGHT;
        }
        // if Input.is_action_pressed("move_left"):
        if input.is_action_pressed("move_left") {
            //direction.x -= 1
            direction += Vector3::LEFT;
        }
        // if Input.is_action_pressed("move_back"):
        if input.is_action_pressed("move_back") {
            // direction.z += 1
            direction += Vector3::BACK;
        }
        // if Input.is_action_pressed("move_forward"):
        if input.is_action_pressed("move_forward") {
            // direction.z -= 1
            direction += Vector3::FORWARD;
        }

        // if direction != Vector3.ZERO:
        if direction != Vector3::ZERO {
            // In the lines below, we turn the character when moving and make the animation play faster.
            //direction = direction.normalized()
            direction = direction.normalized();

            // take the pivot node and rotate it to face the direction we're moving in
            let mut pivot = self.base_mut().get_node_as::<Node3D>("Pivot");
            // $Pivot.basis = Basis.looking_at(direction) (GDScript)
            pivot.set_basis(Basis::looking_at(-direction, Vector3::UP, true));
            // $AnimationPlayer.speed_scale = 4
            self.base()
                .get_node_as::<AnimationPlayer>("AnimationPlayer")
                .set_speed_scale(4.0);
        } else {
            // $AnimationPlayer.speed_scale = 1
            self.base()
                .get_node_as::<AnimationPlayer>("AnimationPlayer")
                .set_speed_scale(1.0);
        }

        // Ground Velocity
        // velocity.x = direction.x * speed
        self.target_velocity.x = direction.x * self.speed;
        //velocity.z = direction.z * speed
        self.target_velocity.z = direction.z * self.speed;

        // jumping
        // if is_on_floor() and Input.is_action_just_pressed("jump"):
        if self.base().is_on_floor() && input.is_action_just_pressed("jump") {
            //velocity.y += jump_impulse
            self.target_velocity.y = self.jump_impulse;
        }

        // We apply gravity every frame so the character always collides with the ground when moving.
        // This is necessary for the is_on_floor() function to work as a body can always detect
        // the floor, walls, etc. when a collision happens the same frame.
        if !self.base().is_on_floor() {
            // velocity.y -= fall_acceleration * delta
            self.target_velocity.y -= self.fall_acceleration * _delta as f32;
        }
        // moving the Character
        let velocity = self.target_velocity;
        self.base_mut().set_velocity(velocity);
        // move_and_slide()
        self.base_mut().move_and_slide();

        // Here, we check if we landed on top of a mob and if so, we kill it and bounce.
        // With move_and_slide(), Godot makes the body move sometimes multiple times in a row to
        // smooth out the character's motion. So we have to loop over all collisions that may have
        // happened.
        // If there are no "slides" this frame, the loop below won't run.
        // for index in range(get_slide_collision_count()):
        for index in 0..self.base().get_slide_collision_count() {
            // var collision = get_slide_collision(index)
            let collision = self.base_mut().get_slide_collision(index).unwrap();
            // if collision.get_collider().is_in_group("mob"):
            if let Some(collider) = collision.get_collider() {
                if let Some(node) = collider.try_cast::<Node3D>().ok() {
                    if node.is_in_group("mob") {
                        // var mob = collision.get_collider()
                        let mut mob = collision.get_collider().unwrap().cast::<Mob>();
                        // if Vector3.UP.dot(collision.get_normal()) > 0.1:
                        if Vector3::UP.dot(collision.get_normal()) > 0.1 {
                            // mob.squash()
                            mob.bind_mut().squash();
                            // velocity.y = bounce_impulse
                            self.target_velocity.y = self.bounce_impulse;
                            // Prevent this block from running more than once,
                            // which would award the player more than 1 point for squashing a single mob.
                            break;
                        }
                    }
                }
            }
        }
        // This makes the character follow a nice arc when jumping
        // $Pivot.rotation.x = PI / 6 * velocity.y / jump_impulse
        let mut pivot = self.base().get_node_as::<Node3D>("Pivot");
        let mut pivot_rotation = pivot.get_rotation();
        pivot_rotation.x = FRAC_PI_6 * self.base().get_velocity().y / self.jump_impulse;
        pivot.set_rotation(pivot_rotation);
    }
}

#[godot_api]
impl Player {
    // signal hit
    #[signal]
    pub fn hit();

    #[func]
    pub fn die(&mut self) {
        // hit.emit()
        self.signals().hit().emit();
        // queue_free()
        self.base_mut().queue_free();
    }

    #[func]
    pub fn on_mob_detector_body_entered(&mut self, _body: Gd<CharacterBody3D>) {
        let mut collision_shape = self
            .base()
            .get_node_as::<CollisionShape3D>("CollisionShape3D");
        collision_shape.set_deferred("disabled", &true.to_variant());
        // die()
        self.die();
    }
}
