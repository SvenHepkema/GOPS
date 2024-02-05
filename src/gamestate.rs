use crate::cards::{Card, CardStack};
use core::cmp::Ordering;

pub struct GameState {
    write_to_console: bool,

    diamonds: CardStack,
    pub diamonds_on_bid: Vec<Card>,

    pub player_a_cards: CardStack,
    pub player_b_cards: CardStack,

    pub player_a_diamonds: Vec<Card>,
    pub player_b_diamonds: Vec<Card>,
}

impl GameState {
    pub fn show_round(&self) {
        if self.write_to_console {
            println!("===============");
            println!("Round {0}", 14 - self.diamonds.cards_in_stack.len());
            println!("===============");
        }
    }

    pub fn show_bid_result(&self, card_a: Card, card_b: Card) {
        if self.write_to_console {
            print!("{}", match (card_a as i32).cmp(&(card_b as i32)) {
                Ordering::Greater => "A WINS",
                Ordering::Equal => "DRAW",
                Ordering::Less => "B WINS",
            });

            println!(
                "\t->\tA: {0}\tB: {1}",
                card_a as i32 + 1,
                card_b as i32 + 1
            );
        }
    }

    pub fn new(write_to_console: bool) -> Self {
        GameState {
            write_to_console,
            diamonds: CardStack::new(),
            diamonds_on_bid: vec![],

            player_a_cards: CardStack::new(),
            player_b_cards: CardStack::new(),

            player_a_diamonds: vec![],
            player_b_diamonds: vec![],
        }
    }

    pub fn draw_diamond(&mut self) {
        self.diamonds_on_bid.push(self.diamonds.draw_random());
    }

    pub fn bid(&mut self, card_a: Card, card_b: Card) {
        self.show_bid_result(card_a, card_b);

        match (card_a as i32).cmp(&(card_b as i32)) {
            Ordering::Greater => self.player_a_diamonds.append(&mut self.diamonds_on_bid),
            Ordering::Less => self.player_b_diamonds.append(&mut self.diamonds_on_bid),
            Ordering::Equal => {},
        };
    }
 
    pub fn is_finished(&self) -> bool {
        self.diamonds.is_empty() && self.diamonds_on_bid.is_empty()
    }
}

pub fn calculate_score(cards: &Vec<Card>) -> i32 {
    let mut sum = 0;

    for card in cards.iter() {
        sum += *card as i32;
    }

    sum
}

