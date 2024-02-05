use crate::cards::{Card, CardStack, show_card_vector};
use core::cmp::Ordering;

#[derive(Clone)]
pub struct GameState {
    write_to_console: bool,

    diamonds: CardStack,
    pub diamonds_on_bid: Vec<Card>,

    pub player_a_cards: CardStack,
    pub player_b_cards: CardStack,

    pub player_a_diamonds: Vec<Card>,
    pub player_b_diamonds: Vec<Card>,
}

fn calculate_score(cards: &Vec<Card>) -> i32 {
    let mut sum = 0;

    for card in cards.iter() {
        sum += *card as i32 + 1;
    }

    sum
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

    pub fn show_result(&self, score_a: i32, score_b: i32, score: i32) {
        if self.write_to_console {
            println!("===============");
            println!("Result");
            println!("===============");

            show_card_vector("A diamonds: ", &self.player_a_diamonds);
            show_card_vector("B diamonds: ", &self.player_b_diamonds);
            println!("Score A: {}", score_a);
            println!("Score B: {}", score_b);
            println!("Score: {0}", score);
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
        self.player_a_cards.draw_card(card_a);
        self.player_b_cards.draw_card(card_b);

        self.show_bid_result(card_a, card_b);

        match (card_a as i32).cmp(&(card_b as i32)) {
            Ordering::Greater => self.player_a_diamonds.append(&mut self.diamonds_on_bid),
            Ordering::Less => self.player_b_diamonds.append(&mut self.diamonds_on_bid),
            Ordering::Equal => {},
        };
    }
 
    pub fn is_finished(&self) -> bool {
        self.player_a_cards.cards_in_stack.is_empty()
    }

    pub fn calculate_score(&self) -> i32 {
        let score_a = calculate_score(&self.player_a_diamonds);
        let score_b = calculate_score(&self.player_b_diamonds);
        let score = score_a - score_b;

        self.show_result(score_a, score_b, score);

        score
    }
}
