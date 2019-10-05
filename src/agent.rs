mod dealer;
mod player;

use crate::{
    card::{Card, Hand},
    TERM,
};

pub use {dealer::Dealer, player::Player};

pub enum Action {
    Stay,
    Hit,
}

pub trait Strategy {
    fn action(&self, top_card: Option<&Card>) -> std::io::Result<Action>;
}

pub trait Agent: Strategy {
    fn hand(&self) -> &Hand;
    fn mut_hand(&mut self) -> &mut Hand;
    fn name(&self) -> &'static str;

    fn val(&self) -> usize {
        self.hand().val()
    }

    fn deal(&mut self, card: Card) {
        self.mut_hand().add(card);
    }

    fn print_hand(&self) -> std::io::Result<()> {
        TERM.write_line(self.hand().hand_string().as_str())?;
        Ok(())
    }

    fn print_hand_hidden(&self) -> std::io::Result<()> {
        TERM.write_line(self.hand().hidden_hand_string().as_str())?;
        Ok(())
    }
}
