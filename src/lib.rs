use console::Term;

mod agent;
mod card;
mod deck;
mod game;

lazy_static::lazy_static! {
    static ref TERM: Term = Term::stdout();
}

pub use crate::game::Game;
