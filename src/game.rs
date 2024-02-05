use crate::cards::Card;
use crate::gamestate::GameState;
use crate::aiplayers::RandomAIPlayer;
use crate::player::Player;

pub struct Game {
    player_a: Box<dyn Player>,
    player_b: Box<dyn Player>,
    state: GameState,
}

impl Game {
    pub fn new(
        player_a: Box<dyn Player>,
        player_b: Box<dyn Player>,
        write_to_console: bool
    ) -> Self {
        Game {
            player_a,
            player_b,
            state: GameState::new(write_to_console),
        }
    }

    pub fn mc_move_and_copy(state: &GameState, mc_is_player_a: bool, card: Card) -> Self {
        let mut state_copy = state.clone();
        state_copy.write_to_console = false;

        let player_a = Box::new(RandomAIPlayer{value: 1});
        let player_b = Box::new(RandomAIPlayer{value: 1});

        match mc_is_player_a {
            true => {
                state_copy.bid(card, player_b.pick_card(false, &state_copy));
            },
            false => {
                state_copy.bid(player_a.pick_card(true, &state_copy), card);
            },
        }

        Game {
            player_a,
            player_b,
            state: state_copy,
        }
    }

    pub fn play(&mut self) -> i32 {
        while !self.state.is_finished() {
            self.state.show_round();

            self.state.draw_diamond();

            let card_a = self.player_a.pick_card(true, &mut self.state);
            let card_b = self.player_b.pick_card(false, &mut self.state);

            self.state.bid(card_a, card_b);
        }


        self.state.calculate_score()
    }
}
