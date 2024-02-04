use rand::prelude::{thread_rng, IteratorRandom};
use strum::{EnumIter, IntoEnumIterator};

use crate::player::Player;

#[derive(Debug, EnumIter, PartialOrd, PartialEq, Clone, Copy)]
pub enum Card {
    Ace,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    C10,
    Jack,
    Queen,
    King,
}

pub struct CardStack {
    pub cards_in_stack: Vec<Card>,
    pub cards_out_of_stack: Vec<Card>,
}

impl CardStack {
    pub fn new() -> Self {
        CardStack {
            cards_in_stack: Card::iter().collect(),
            cards_out_of_stack: vec![],
        }
    }

    pub fn draw_random(&mut self) -> Card {
        let (i, &out) = self
            .cards_in_stack
            .iter()
            .enumerate()
            .choose(&mut thread_rng())
            .unwrap();
        self.cards_in_stack.remove(i);
        self.cards_out_of_stack.push(out.clone());
        out
    }

    pub fn draw_card(&mut self, card: Card) -> Card {
        if self.cards_in_stack.contains(&card) {
            let index = self.cards_in_stack.iter().position(|x| *x == card).unwrap();
            self.cards_in_stack.remove(index);
            self.cards_out_of_stack.push(card);
            card
        } else {
            panic!("Tried to draw card that is not in the stack.");
        }
    }

    pub fn is_empty(&self) -> bool {
        return self.cards_in_stack.is_empty();
    }
}

pub struct GameState {
    diamonds: CardStack,
    pub diamonds_on_bid: Vec<Card>,

    pub player_a_cards: CardStack,
    pub player_b_cards: CardStack,

    player_a_diamonds: Vec<Card>,
    player_b_diamonds: Vec<Card>,
}

impl GameState {
    pub fn new() -> Self {
        let mut diamonds = CardStack::new();
        let diamonds_on_bid = vec![diamonds.draw_random()];

        GameState {
            diamonds,
            diamonds_on_bid,

            player_a_cards: CardStack::new(),
            player_b_cards: CardStack::new(),

            player_a_diamonds: vec![],
            player_b_diamonds: vec![],
        }
    }

    pub fn bid(&mut self, card_a: Card, card_b: Card) {
        print!("While playing for: ");

        for card in self.diamonds_on_bid.iter() {
            print!("{0} _ ", *card as i32 + 1);
        }

        println!("A bode: {0} and B bode: {1}", card_a as i32 + 1, card_b as i32 + 1);

        if card_a != card_b {
            if card_a > card_b {
                self.player_a_diamonds.append(&mut self.diamonds_on_bid);
            } else {
                self.player_b_diamonds.append(&mut self.diamonds_on_bid);
            }
        }

        self.diamonds_on_bid.push(self.diamonds.draw_random());
    }
}

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
