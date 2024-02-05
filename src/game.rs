use crate::cards::Card;
use crate::gamestate::GameState;
use crate::player::Player;

pub fn calculate_score(cards: &Vec<Card>) -> i32 {
    let mut sum = 0;

    for card in cards.iter() {
        sum += *card as i32;
    }

    sum
}

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

    pub fn play(&mut self) -> i32 {
        while !self.state.is_finished() {
            self.state.show_round();

            self.state.draw_diamond();

            let card_a = self.player_a.play(true, &mut self.state);
            let card_b = self.player_b.play(false, &mut self.state);

            self.state.bid(card_a, card_b);
        }

        let score_a = calculate_score(&self.state.player_a_diamonds);
        let score_b = calculate_score(&self.state.player_b_diamonds);

        score_a - score_b
    }
}
