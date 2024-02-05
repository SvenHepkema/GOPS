use std::io;

use crate::cards::{Card, show_card_vector, cast_number_to_card, show_card_vector_with_spaces};
use crate::gamestate::GameState;
use crate::player::Player;

pub struct HumanPlayer {
    pub value: i32,
}

fn get_number_from_user() -> i32 {
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();

    match trimmed.parse::<i32>() {
        Ok(i) => i,
        Err(..) => {
            println!("this was not an integer: {}", trimmed);
            get_number_from_user()
        }
    }
}


impl Player for HumanPlayer {
    fn pick_card(&self, is_player_a: bool, state: &GameState) -> Card {
        let your_cards = match is_player_a {
            true => &state.player_a_cards.cards_in_stack,
            false => &state.player_b_cards.cards_in_stack,
        };
        let your_diamonds = match is_player_a {
            true => &state.player_a_diamonds,
            false => &state.player_b_diamonds,
        };

        show_card_vector("At stake\t: ", &state.diamonds_on_bid);
        show_card_vector_with_spaces("Your cards\t: ", your_cards);
        show_card_vector("Your diamonds\t: ", your_diamonds);

        let mut card = cast_number_to_card(get_number_from_user());
        while card == None || !your_cards.contains(&card.unwrap()) {
            print!("Pick one: ");
            card = cast_number_to_card(get_number_from_user());
        }

        card.unwrap()
    }
}
