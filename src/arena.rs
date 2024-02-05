use crate::aiplayers::{EqualBuyer, RandomAIPlayer, SimpleMC};
use crate::game::Game;
use crate::player::Player;
use core::cmp::Ordering;

// TODO: The actual players should be stateless, so functions
// The bojects should be replaced by enums in the rest of the code
pub enum AIEnum {
    Random,
    EqualBuyer,
    SimpleMC,
}

fn ai_player_factory(ai: &AIEnum) -> Box<dyn Player> {
    match ai {
        AIEnum::Random => Box::new(RandomAIPlayer { value: 1 }),
        AIEnum::EqualBuyer => Box::new(EqualBuyer { value: 1 }),
        AIEnum::SimpleMC => Box::new(SimpleMC { value: 1 }),
    }
}

fn ai_get_name(ai: &AIEnum) -> &str {
    match ai {
        AIEnum::Random => "Random",
        AIEnum::EqualBuyer => "EqualBuyer",
        AIEnum::SimpleMC => "SimpleMC",
    }
}

pub struct Arena {
    n_games: u32,
    player_a: AIEnum,
    player_b: AIEnum,
    player_a_wins: u32,
    player_b_wins: u32,
    draws: u32,
}

impl Arena {
    pub fn new(n_games: u32, player_a: AIEnum, player_b: AIEnum) -> Self {
        Arena {
            n_games,
            player_a,
            player_b,
            player_a_wins: 0,
            player_b_wins: 0,
            draws: 0,
        }
    }

    // Highly inefficient function TODO: Remove useless reinstantiations
    pub fn start_tournament(&mut self) {
        let mut games = vec![];

        for _ in 0..self.n_games {
            games.push(Game::new(
                ai_player_factory(&self.player_a),
                ai_player_factory(&self.player_b),
                false,
            ))
        }

        // TODO: Multithread the execution of the games
        for game in games.iter_mut() {
            let result = game.play();

            match result.cmp(&0) {
                Ordering::Greater => self.player_a_wins += 1,
                Ordering::Equal => self.draws += 1,
                Ordering::Less => self.player_b_wins += 1,
            }
        }
    }

    pub fn show_results(&self) {
        println!(
            "Player A: {} \twon {} games ({:.2}%)",
            ai_get_name(&self.player_a),
            self.player_a_wins,
            self.player_a_wins / self.n_games
        );
        println!(
            "Player B: {} \twon {} games ({:.2}%)",
            ai_get_name(&self.player_b),
            self.player_b_wins,
            self.player_b_wins / self.n_games
        );
        println!(
            "There were {} draws ({:.2}%)",
            self.draws,
            self.draws / self.n_games
        );
    }
}
