use crate::cards::Card;
use crate::gamestate::GameState;

pub trait Player {
    fn play(&self, is_player_a: bool, state: &mut GameState) -> Card;
}
