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
