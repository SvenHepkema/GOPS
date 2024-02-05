use rand::prelude::{thread_rng, IteratorRandom};
use strum::{EnumIter, IntoEnumIterator};

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

pub fn cast_number_to_card(number: i32) -> Option<Card> {
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
        _ => None,
    }
}

#[derive(Clone)]
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

    pub fn pick_random(&self) -> Card {
        let (_, &out) = self
            .cards_in_stack
            .iter()
            .enumerate()
            .choose(&mut thread_rng())
            .unwrap();
        out
    }

    pub fn draw_random(&mut self) -> Card {
        let card = self.pick_random();
        self.draw_card(card)
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

pub fn show_card_vector(message: &str, cards: &Vec<Card>) {
    print!("{}", message);
    for card in cards.iter() {
        print!("{0}  ", *card as i32 + 1);
    }
    println!("");
}

pub fn show_card_vector_with_spaces(message: &str, cards: &Vec<Card>) {
    print!("{}", message);
    for number in 1..14 {
        if cards.contains(&cast_number_to_card(number).unwrap()) {
            print!("{0}  ", number);
        } else {
            if number >= 10 {
                print!("    ");
            }
            else {
                print!("   ");
            }
        }
    }
    println!("");
}
