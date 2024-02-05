use crate::cards::Card;
use crate::gamestate::GameState;

pub trait Player {
    fn pick_card(&self, is_player_a: bool, state: &GameState) -> Card;
}
