use crate::game::{Card, GameState};
use crate::player::Player;

pub struct RandomAIPlayer {
    pub value: i32
}

impl Player for RandomAIPlayer {
    fn play(&self, is_player_a: bool, state: &mut GameState) -> Card {
        match is_player_a {
            true => state.player_a_cards.draw_random(),
            false => state.player_b_cards.draw_random(),
        }
    }
}

pub struct EqualBuyer {
    pub value: i32
}

impl Player for EqualBuyer {
    fn play(&self, is_player_a: bool, state: &mut GameState) -> Card {
        match is_player_a {
            true => state.player_a_cards.draw_card(*state.diamonds_on_bid.last().unwrap()),
            false => state.player_b_cards.draw_card(*state.diamonds_on_bid.last().unwrap()),
        }
    }
}
