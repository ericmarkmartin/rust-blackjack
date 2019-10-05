use dialoguer::Select;

use crate::{
    card::{Card, Hand},
    TERM,
};

use super::{Action, Agent, Strategy};

pub struct Player {
    hand: Hand,
}

impl Player {
    pub fn new() -> Self {
        Player { hand: Hand::new() }
    }

    fn print_status(&self, top_card: Option<&Card>) -> std::io::Result<()> {
        if let Some(top_card) = top_card {
            TERM.write_line(format!("Dealer showing {}", top_card).as_str())?;
        }
        TERM.write_line("Player cards:")?;
        self.print_hand()?;
        TERM.write_line(format!("Current value: {}", self.val()).as_str())?;

        Ok(())
    }
}

impl Strategy for Player {
    fn action(&self, top_card: Option<&Card>) -> std::io::Result<Action> {
        self.print_status(top_card)?;
        let index = Select::new()
            .with_prompt("Pick an action")
            .default(0)
            .item("Stay")
            .item("Hit")
            .interact_on(&TERM)?;

        if index == 0 {
            Ok(Action::Stay)
        } else {
            Ok(Action::Hit)
        }
    }
}

impl Agent for Player {
    fn hand(&self) -> &Hand {
        &self.hand
    }

    fn mut_hand(&mut self) -> &mut Hand {
        &mut self.hand
    }

    fn name(&self) -> &'static str {
        "Player"
    }
}
