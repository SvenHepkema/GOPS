use crate::cards::{Card, CardStack};

pub struct GameState {
    diamonds: CardStack,
    pub diamonds_on_bid: Vec<Card>,

    pub player_a_cards: CardStack,
    pub player_b_cards: CardStack,

    pub player_a_diamonds: Vec<Card>,
    pub player_b_diamonds: Vec<Card>,
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

        println!(
            "A bode: {0} and B bode: {1}",
            card_a as i32 + 1,
            card_b as i32 + 1
        );

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
