use rand::{seq::SliceRandom, thread_rng};

use crate::{
    agent::Agent,
    card::{Card, ALL_CARDS},
};

pub struct Deck {
    cards: Vec<Card>,
    discard: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        Deck {
            cards: ALL_CARDS.to_vec(),
            discard: Vec::new(),
        }
    }

    pub fn new_shuffled() -> Self {
        let mut deck = Self::new();
        deck.shuffle();
        deck
    }

    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    pub fn deal<T: Agent>(&mut self, agent: &mut T) -> Card {
        let card = match self.cards.pop() {
            Some(card) => card,
            None => {
                self.recycle();
                self.cards.pop().unwrap()
            }
        };
        agent.deal(card);
        card
    }

    pub fn deal_2<T: Agent>(&mut self, agent: &mut T) -> Card {
        self.deal(agent);
        self.deal(agent)
    }

    pub fn discard<T: Agent>(&mut self, agent: &mut T) {
        self.discard.append(agent.mut_hand().mut_cards());
    }

    fn recycle(&mut self) {
        self.cards.append(&mut self.discard);
        self.shuffle();
    }
}
