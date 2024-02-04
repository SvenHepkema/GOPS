use std::io;

use crate::game::{Card, GameState};
use crate::player::Player;

pub struct HumanPlayer {
    pub value: i32
}

fn get_number_from_user() -> i32 {
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();

    match trimmed.parse::<i32>() {
        Ok(i) => {
            i
        },
        Err(..) => {
            println!("this was not an integer: {}", trimmed);
            get_number_from_user()
        },
    }
}

fn cast_number_to_card(number: i32) -> Option<Card> {
    match number {
        1 => Some(Card::Ace),
        2 => Some(Card::C2),
        3 => Some(Card::C3),
        4 => Some(Card::C4),
        5 => Some(Card::C5),
        6 => Some(Card::C6),
        7 => Some(Card::C7),
        8 => Some(Card::C8),
        9 => Some(Card::C9),
        10 => Some(Card::C10),
        11 => Some(Card::Jack),
        12 => Some(Card::Queen),
        13 => Some(Card::King),
        _ => None
    }
}


impl Player for HumanPlayer {
    fn play(&self, is_player_a: bool, state: &mut GameState) -> Card {
        println!("Playing for:");

        for card in state.diamonds_on_bid.iter() {
            print!("{0} _ ", *card as i32 + 1);
        }

        println!("");

        let cards = match is_player_a {
            true => &state.player_a_cards.cards_in_stack,
            false => &state.player_b_cards.cards_in_stack,
        };

        for card in cards.iter() {
            print!("{0} _ ", *card as i32 + 1);
        }

        println!("Pick one: ");
        let mut card = cast_number_to_card(get_number_from_user());
        while card == None || !cards.contains(&card.unwrap()) {
            println!("Pick one: ");
            card = cast_number_to_card(get_number_from_user());
        }

        let chosen_card = card.unwrap();

        println!("You chose to play: {0}", chosen_card as i32 + 1);

        
        match is_player_a {
            true => state.player_a_cards.draw_card(chosen_card),
            false => state.player_b_cards.draw_card(chosen_card),
        }
    }
}
