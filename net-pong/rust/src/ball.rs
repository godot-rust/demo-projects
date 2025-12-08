use crate::pong::Pong;
use godot::classes::{Area2D, IArea2D};
use godot::prelude::*;

const DEFAULT_SPEED: f32 = 100.0;

#[derive(GodotClass)]
#[class(init, base=Area2D)]
pub struct Ball {
    #[init(val = Vector2::LEFT)]
    direction: Vector2,
    stopped: bool,
    #[init(val = DEFAULT_SPEED)]
    speed: f32,
    base: Base<Area2D>,
}

#[godot_api]
impl IArea2D for Ball {
    fn process(&mut self, delta: f32) {
        let screen_size = self.base().get_viewport_rect().size;
        self.speed += delta;

        if !self.stopped {
            // Ball will move normally for both players,
            // even if it's sightly out of sync between them,
            // so each player sees the motion as smooth and not jerky.
            let direction = self.direction;
            let translation = direction * self.speed * delta;
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
        // Use base_mut() to allow for reentrancy – required if a game stops, and we need to reset our ball.
        let mut guard = self.base_mut();
        if guard.is_multiplayer_authority() {
            // Only the master will decide when the ball is out on
            // the left side (its own side). This makes the game
            // playable even if latency is high and ball is going
            // fast. Otherwise, the ball might be out in the other
            // player's screen but not this one.
            if ball_pos.x < 0.0 {
                let args = vslice![false];
                parent.rpc("update_score", args);
                guard.rpc("reset_ball", args);
            }
        } else {
            // Only the puppet will decide when the ball is out on
            // the right side, which is its own side. This makes
            // the game playable even if latency is high and ball
            // is going fast. Otherwise, the ball might be out in the
            // other player's screen but not this one.
            if ball_pos.x > screen_size.x {
                let args = vslice![true];
                parent.rpc("update_score", args);
                guard.rpc("reset_ball", args);
            }
        }
    }
}

#[godot_api]
impl Ball {
    #[rpc(any_peer, call_local)]
    fn bounce(&mut self, is_left: bool, random: f32) {
        // Using sync because both players can make it bounce.
        if is_left {
            self.direction.x = self.direction.x.abs();
        } else {
            self.direction.x = -self.direction.x.abs();
        }
        self.speed *= 1.1;
        self.direction.y = random * 2.0 - 1.0;
        self.direction = self.direction.normalized();
    }

    #[rpc(any_peer, call_local)]
    fn stop(&mut self) {
        self.stopped = true;
    }

    #[rpc(any_peer, call_local)]
    fn reset_ball(&mut self, for_left: bool) {
        let screen_center = self.base().get_viewport_rect().size / 2.0;
        self.base_mut().set_position(screen_center);
        if for_left {
            self.direction = Vector2::LEFT;
        } else {
            self.direction = Vector2::RIGHT;
        }
        self.speed = DEFAULT_SPEED;
    }
}
