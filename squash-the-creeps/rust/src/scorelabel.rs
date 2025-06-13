use godot::classes::{Control, Label};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Control)]
pub struct UserInterface {
    score: u32,
    base: Base<Control>,
}

#[godot_api]
impl UserInterface {
    #[func]
    pub fn on_mob_squashed(&mut self) {
        self.score += 1;

        let mut label = self.base().get_node_as::<Label>("ScoreLabel");
        label.set_text(format!("Score: {}", self.score).as_str());
    }
}
