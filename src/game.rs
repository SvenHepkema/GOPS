use crate::gamestate::GameState;
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

    pub fn play(&mut self) -> i32 {
        while !self.state.is_finished() {
            self.state.show_round();

            self.state.draw_diamond();

            let card_a = self.player_a.play(true, &mut self.state);
            let card_b = self.player_b.play(false, &mut self.state);

            self.state.bid(card_a, card_b);
        }


        self.state.calculate_score()
    }
}
