/*
extends Area2D

const MOTION_SPEED = 150

@export var left := false

var _motion := 0.0
var _you_hidden := false

@onready var _screen_size_y := get_viewport_rect().size.y

func _process(delta: float) -> void:
    # Is the master of the paddle.
    if is_multiplayer_authority():
        _motion = Input.get_axis(&"move_up", &"move_down")

        if not _you_hidden and _motion != 0:
            _hide_you_label()

        _motion *= MOTION_SPEED

        # Using unreliable to make sure position is updated as fast
        # as possible, even if one of the calls is dropped.
        set_pos_and_motion.rpc(position, _motion)
    else:
        if not _you_hidden:
            _hide_you_label()

    translate(Vector2(0.0, _motion * delta))

    # Set screen limits.
    position.y = clampf(position.y, 16, _screen_size_y - 16)


# Synchronize position and speed to the other peers.
@rpc("unreliable")
func set_pos_and_motion(pos: Vector2, motion: float) -> void:
    position = pos
    _motion = motion


func _hide_you_label() -> void:
    _you_hidden = true
    $You.hide()


func _on_paddle_area_enter(area: Area2D) -> void:
    if is_multiplayer_authority():
        # Random for new direction generated checked each peer.
        area.bounce.rpc(left, randf())
*/

use godot::classes::{Area2D, Input, Label};
use godot::global::randf;
use godot::prelude::*;

const MOTION_SPEED: f32 = 150.0;

#[derive(GodotClass)]
#[class(base=Area2D)]
struct Paddle {
    #[export]
    left: bool,
    _motion: f32,
    _you_hidden: bool,
    #[export]
    you_label: Option<Gd<Label>>,

    base: Base<Area2D>,
}

use godot::classes::IArea2D;

use crate::variant_array_to_vec;

#[godot_api]
impl IArea2D for Paddle {
    fn init(base: Base<Area2D>) -> Self {
        godot_print!("Hello, world!"); // Prints to the Godot console

        Self {
            left: false,
            _motion: 0.0,
            _you_hidden: false,
            you_label: None,
            base,
        }
    }

    fn ready(&mut self) {
        // bounce signal is emitted when the ball enters the paddle area.
        self.signals()
            .area_entered()
            .connect_self(|this: &mut Self, mut area: Gd<Area2D>| {
                if this.base().is_multiplayer_authority() {
                    // Random for new direction generated checked each peer.
                    let args = varray![this.left, randf()];
                    let varargs = variant_array_to_vec(args);
                    area.rpc("bounce", varargs.as_slice());
                }
            });
    }

    /*
       func _process(delta: float) -> void:
       # Is the master of the paddle.
       if is_multiplayer_authority():
           _motion = Input.get_axis(&"move_up", &"move_down")

           if not _you_hidden and _motion != 0:
               _hide_you_label()

           _motion *= MOTION_SPEED

           # Using unreliable to make sure position is updated as fast
           # as possible, even if one of the calls is dropped.
           set_pos_and_motion.rpc(position, _motion)
       else:
           if not _you_hidden:
               _hide_you_label()

       translate(Vector2(0.0, _motion * delta))

       # Set screen limits.
       position.y = clampf(position.y, 16, _screen_size_y - 16)
    */
    fn process(&mut self, delta: f64) {
        if self.base().is_multiplayer_authority() {
            let input = Input::singleton();
            self._motion = input.get_axis("move_up", "move_down");

            if !self._you_hidden && self._motion != 0.0 {
                self.you_label.as_mut().unwrap().hide();
            }

            self._motion *= MOTION_SPEED;

            // Using unreliable to make sure position is updated as fast
            // as possible, even if one of the calls is dropped.
            let args = varray![self.base().get_position(), self._motion];
            let varargs = variant_array_to_vec(args);
            self.base_mut()
                .rpc("set_pos_and_motion", varargs.as_slice());
        } else {
            if !self._you_hidden {
                self.you_label.as_mut().unwrap().hide();
            }
        }

        let translation = Vector2::new(0.0, self._motion * delta as f32);

        self.base_mut().translate(translation);

        // Set screen limits.
        let screen_size_y = self.base().get_viewport_rect().size.y;
        let position = self.base().get_position();
        self.base_mut().set_position(Vector2::new(
            position.x,
            position.y.clamp(16.0, screen_size_y - 16.0),
        ));
    }
}

#[godot_api]
impl Paddle {
    /*
    # Synchronize position and speed to the other peers.
    @rpc("unreliable")
    func set_pos_and_motion(pos: Vector2, motion: float) -> void:
        position = pos
        _motion = motion
     */
    #[rpc(unreliable)]
    fn set_pos_and_motion(&mut self, pos: Vector2, motion: f32) {
        self.base_mut().set_position(pos);
        self._motion = motion;
    }
}
