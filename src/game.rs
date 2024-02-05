use crate::player::Player;
use crate::cards::Card;
use crate::gamestate::GameState;

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
    pub fn new(player_a: Box<dyn Player>, player_b: Box<dyn Player>) -> Self {
        Game {
            player_a,
            player_b,
            state: GameState::new(),
        }
    }

    pub fn play(&mut self) -> i32 {
        let mut counter = 1;
        while counter <= 13 {
            println!("===============");
            println!("Round {0}", counter);
            println!("===============");
            let card_a = self.player_a.play(true, &mut self.state);
            let card_b = self.player_b.play(false, &mut self.state);

            if counter != 13 {
                self.state.bid(card_a, card_b);
            }
            counter += 1;
        }

        let score_a = calculate_score(&self.state.player_a_diamonds);
        let score_b = calculate_score(&self.state.player_b_diamonds);

        score_a - score_b
    }
}
