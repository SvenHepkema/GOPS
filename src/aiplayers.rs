use crate::cards::Card;
use crate::gamestate::GameState;
use crate::player::Player;

pub struct RandomAIPlayer {
    pub value: i32
}

impl Player for RandomAIPlayer {
    fn pick_card(&self, is_player_a: bool, state: &GameState) -> Card {
        match is_player_a {
            true => state.player_a_cards.pick_random(),
            false => state.player_b_cards.pick_random(),
        }
    }
}

pub struct EqualBuyer {
    pub value: i32
}

impl Player for EqualBuyer {
    fn pick_card(&self, _: bool, state: &GameState) -> Card {
        *state.diamonds_on_bid.last().unwrap()
    }
}

pub struct SimpleMC {
    pub value: i32
}

impl Player for SimpleMC {
    fn pick_card(&self, is_player_a: bool, state: &GameState) -> Card {
        *state.diamonds_on_bid.last().unwrap()
        /*
        let options = match is_player_a {
            true => &state.player_a_cards.cards_in_stack,
            false => &state.player_b_cards.cards_in_stack,
        }.clone();

    
        let best_card = options[0];


        match is_player_a {
            true => state.player_a_cards.draw_card(best_card),
            false => state.player_b_cards.draw_card(best_card),
        }
        */
    }
}
