use crate::cards::Card;
use crate::game::Game;
use crate::gamestate::GameState;
use crate::player::Player;
use rayon::prelude::*;

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
    pub n_samples: i32
}

impl Player for SimpleMC {
    fn pick_card(&self, is_player_a: bool, state: &GameState) -> Card {
        let options = match is_player_a {
            true => &state.player_a_cards.cards_in_stack,
            false => &state.player_b_cards.cards_in_stack,
        }.clone();

    
        let mut best_card = options[0];
        let mut best_score: i32 = 0;

        for card in options.iter() {
            let sum_score = (0..self.n_samples)
                .into_par_iter()
                .map(|_| { Game::mc_move_and_copy(state, is_player_a, *card).play() })
                .sum();

            if (sum_score > best_score && is_player_a) ||
               (sum_score < best_score && !is_player_a) {
                best_card = card.clone();
                best_score = sum_score;
            }
        }

        best_card
    }
}
