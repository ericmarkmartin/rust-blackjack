use super::{Action, Agent, Strategy};
use crate::card::{Card, Hand};

pub struct Dealer {
    hand: Hand,
}

impl Dealer {
    pub fn new() -> Self {
        Dealer { hand: Hand::new() }
    }
}

impl Strategy for Dealer {
    fn action(&self, _top_card: Option<&Card>) -> std::io::Result<Action> {
        if self.hand.val() < 17 {
            Ok(Action::Hit)
        } else {
            Ok(Action::Stay)
        }
    }
}

impl Agent for Dealer {
    fn hand(&self) -> &Hand {
        &self.hand
    }

    fn mut_hand(&mut self) -> &mut Hand {
        &mut self.hand
    }

    fn name(&self) -> &'static str {
        "Dealer"
    }
}
