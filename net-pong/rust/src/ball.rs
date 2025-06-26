use godot::classes::Area2D;
use godot::prelude::*;

const DEFAULT_SPEED: f64 = 100.0;

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct Ball {
    direction: Vector2,
    stopped: bool,
    _speed: f64,
    base: Base<Area2D>,
}

use godot::classes::IArea2D;

use crate::pong::Pong;
use crate::variant_array_to_vec;

#[godot_api]
impl IArea2D for Ball {
    fn init(base: Base<Area2D>) -> Self {
        godot_print!("Hello, world!"); // Prints to the Godot console

        Self {
            direction: Vector2::LEFT,
            stopped: false,
            _speed: DEFAULT_SPEED,
            base,
        }
    }

    fn ready(&mut self) {}

    fn process(&mut self, delta: f64) {
        let screen_size = self.base().get_viewport_rect().size;
        self._speed += delta;

        if !self.stopped {
            // Ball will move normally for both players,
            // even if it's sightly out of sync between them,
            // so each player sees the motion as smooth and not jerky.
            let direction = self.direction;
            let translation = direction * (self._speed * delta) as f32;
            self.base_mut().translate(translation);
        }

        // Check screen bounds to make ball bounce.
        let ball_pos = self.base().get_position();
        if (ball_pos.y < 0.0 && self.direction.y < 0.0)
            || (ball_pos.y > screen_size.y && self.direction.y > 0.0)
        {
            self.direction.y = -self.direction.y;
        }

        let mut parent = self.base().get_parent().unwrap().cast::<Pong>();
        if self.base().is_multiplayer_authority() {
            // Only the master will decide when the ball is out in
            // the left side (its own side). This makes the game
            // playable even if latency is high and ball is going
            // fast. Otherwise, the ball might be out in the other
            // player's screen but not this one.
            if ball_pos.x < 0.0 {
                //self.base().get_parent().update_score.rpc(false);
                let args = varray![false];
                let args = variant_array_to_vec(args);
                parent.rpc("update_score", args.as_slice());
                self.base_mut().rpc("reset_ball", args.as_slice());
            }
        } else {
            // Only the puppet will decide when the ball is out in
            // the right side, which is its own side. This makes
            // the game playable even if latency is high and ball
            // is going fast. Otherwise, the ball might be out in the
            // other player's screen but not this one.
            if ball_pos.x > screen_size.x {
                //self.base().get_parent().update_score.rpc(true);
                let args = varray![true];
                let args = variant_array_to_vec(args);
                parent.rpc("update_score", args.as_slice());
                self.base_mut().rpc("reset_ball", args.as_slice());
            }
        }

        /*
           _speed += delta
           # Ball will move normally for both players,
           # even if it's sightly out of sync between them,
           # so each player sees the motion as smooth and not jerky.
           if not stopped:
               translate(_speed * delta * direction)

           # Check screen bounds to make ball bounce.
           var ball_pos := position
           if (ball_pos.y < 0 and direction.y < 0) or (ball_pos.y > _screen_size.y and direction.y > 0):
               direction.y = -direction.y

           if is_multiplayer_authority():
               # Only the master will decide when the ball is out in
               # the left side (its own side). This makes the game
               # playable even if latency is high and ball is going
               # fast. Otherwise, the ball might be out in the other
               # player's screen but not this one.
               if ball_pos.x < 0:
                   get_parent().update_score.rpc(false)
                   _reset_ball.rpc(false)
           else:
               # Only the puppet will decide when the ball is out in
               # the right side, which is its own side. This makes
               # the game playable even if latency is high and ball
               # is going fast. Otherwise, the ball might be out in the
               # other player's screen but not this one.
               if ball_pos.x > _screen_size.x:
                   get_parent().update_score.rpc(true)
                   _reset_ball.rpc(true)
        */
    }
}

#[godot_api]
impl Ball {
    /*
    @rpc("any_peer", "call_local")
    func bounce(left: bool, random: float) -> void:
        # Using sync because both players can make it bounce.
        if left:
            direction.x = abs(direction.x)
        else:
            direction.x = -abs(direction.x)

        _speed *= 1.1
        direction.y = random * 2.0 - 1
        direction = direction.normalized()
    */
    #[rpc(any_peer, call_local)]
    fn bounce(&mut self, is_left: bool, random: f32) {
        // Using sync because both players can make it bounce.
        if is_left {
            self.direction.x = self.direction.x.abs();
        } else {
            self.direction.x = -self.direction.x.abs();
        }
        self._speed *= 1.1;
        self.direction.y = random * 2.0 - 1.0;
        self.direction = self.direction.normalized();
    }

    /*
    @rpc("any_peer", "call_local")
    func stop() -> void:
        stopped = true
     */
    #[rpc(any_peer, call_local)]
    fn stop(&mut self) {
        self.stopped = true;
    }

    /*
    @rpc("any_peer", "call_local")
    func _reset_ball(for_left: float) -> void:
        position = _screen_size / 2
        if for_left:
            direction = Vector2.LEFT
        else:
            direction = Vector2.RIGHT
        _speed = DEFAULT_SPEED
     */

    #[rpc(any_peer, call_local)]
    fn reset_ball(&mut self, for_left: bool) {
        let screen_center = self.base().get_viewport_rect().size / 2.0;
        self.base_mut().set_position(screen_center);
        if for_left {
            self.direction = Vector2::LEFT;
        } else {
            self.direction = Vector2::RIGHT;
        }
        self._speed = DEFAULT_SPEED;
    }
}
