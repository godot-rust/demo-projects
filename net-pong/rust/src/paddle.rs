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
                    let args = vslice![this.left, randf()];
                    area.rpc("bounce", args);
                }
            });
    }

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
            let args = vslice![self.base().get_position(), self._motion];
            self.base_mut().rpc("set_pos_and_motion", args);
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
    #[rpc(unreliable)]
    fn set_pos_and_motion(&mut self, pos: Vector2, motion: f32) {
        self.base_mut().set_position(pos);
        self._motion = motion;
    }
}
