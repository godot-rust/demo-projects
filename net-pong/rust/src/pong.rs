use godot::classes::Area2D;
use godot::classes::Button;
use godot::classes::Label;
use godot::classes::Node2D;
use godot::prelude::*;

const SCORE_TO_WIN: i32 = 10;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct Pong {
    score_left: i32,
    score_right: i32,
    #[export]
    player1: Option<Gd<Area2D>>,
    #[export]
    player2: Option<Gd<Area2D>>,
    #[export]
    score_left_node: Option<Gd<Label>>,
    #[export]
    score_right_node: Option<Gd<Label>>,
    #[export]
    winner_left: Option<Gd<Label>>,
    #[export]
    winner_right: Option<Gd<Label>>,
    #[export]
    exit_game: Option<Gd<Button>>,
    #[export]
    ball: Option<Gd<Ball>>,
    base: Base<Node2D>,
}

use godot::classes::INode2D;

use crate::ball::Ball;

#[godot_api]
impl INode2D for Pong {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            score_left: 0,
            score_right: 0,
            player1: None,
            player2: None,
            score_left_node: None,
            score_right_node: None,
            winner_left: None,
            winner_right: None,
            exit_game: None,
            ball: None,
            base,
        }
    }

    fn ready(&mut self) {
        if self.base().get_multiplayer().unwrap().is_server() {
            // For the server, give control of player 2 to the other peer.
            let authority = self.base().get_multiplayer().unwrap().get_peers()[0];
            self.player2
                .as_mut()
                .unwrap()
                .set_multiplayer_authority(authority);
        } else {
            // For the client, give control of player 2 to itself.
            let authority = self.base().get_multiplayer().unwrap().get_unique_id();
            self.player2
                .as_mut()
                .unwrap()
                .set_multiplayer_authority(authority);
        }

        let gd_ref = self.to_gd();
        self.exit_game
            .as_mut()
            .unwrap()
            .signals()
            .pressed()
            .builder()
            .connect_other_mut(&gd_ref, |this: &mut Self| {
                this._on_exit_game_pressed();
            });
    }
}

#[godot_api]
impl Pong {
    #[signal]
    pub fn game_finished();

    #[rpc(any_peer, call_local)]
    fn update_score(&mut self, add_to_left: bool) {
        if add_to_left {
            self.score_left += 1;
            self.score_left_node
                .as_mut()
                .unwrap()
                .set_text(self.score_left.to_string().as_str());
        } else {
            self.score_right += 1;
            self.score_right_node
                .as_mut()
                .unwrap()
                .set_text(self.score_right.to_string().as_str());
        }

        let mut game_ended = false;
        if self.score_left == SCORE_TO_WIN {
            self.winner_left.as_mut().unwrap().show();
            game_ended = true;
        } else if self.score_right == SCORE_TO_WIN {
            self.winner_right.as_mut().unwrap().show();
            game_ended = true;
        }

        if game_ended {
            self.exit_game.as_mut().unwrap().show();
            self.ball.as_mut().unwrap().rpc("stop", &[]);
        }
    }

    #[func]
    fn _on_exit_game_pressed(&mut self) {
        self.signals().game_finished().emit();
    }
}
