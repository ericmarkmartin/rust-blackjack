use dialoguer::Select;

use crate::{
    agent::{Action, Agent, Dealer, Player},
    card::Card,
    deck::Deck,
    TERM,
};

pub struct Game {
    deck: Deck,
    dealer: Dealer,
    player: Player,
}

impl Game {
    pub fn new() -> Game {
        Game {
            deck: Deck::new_shuffled(),
            dealer: Dealer::new(),
            player: Player::new(),
        }
    }

    fn deal(&mut self) {
        self.deck.deal_2(&mut self.player);
        self.deck.deal_2(&mut self.dealer);
    }

    fn discard(&mut self) {
        self.deck.discard(&mut self.player);
        self.deck.discard(&mut self.dealer);
    }

    pub fn play(&mut self) -> std::io::Result<()> {
        while {
            self.round()?;
            TERM.write_line("")?;
            self.should_play_again()?
        } {
            TERM.write_line("")?;
        }
        Ok(())
    }

    fn should_play_again(&mut self) -> std::io::Result<bool> {
        let index = Select::new()
            .with_prompt("Would you like to play another round?")
            .default(0)
            .item("Yes")
            .item("No")
            .interact_on(&TERM)?;

        Ok(index == 0)
    }

    pub fn round(&mut self) -> std::io::Result<()> {
        self.deal();
        if self.player_play()? && (!self.dealer_play()? || self.dealer.val() < self.player.val()) {
            TERM.write_line("Player wins")?;
        } else if self.dealer.val() == self.player.val() {
            TERM.write_line("Push")?;
        } else {
            TERM.write_line("Dealer wins")?;
        }
        self.discard();
        Ok(())
    }

    pub fn print_status(&self) -> std::io::Result<()> {
        TERM.write_line("Dealer showing:")?;
        self.dealer.print_hand_hidden()?;
        TERM.write_line("Player cards:")?;
        self.player.print_hand()?;
        TERM.write_line(format!("Current value: {}", self.player.val()).as_str())?;
        Ok(())
    }

    fn accumulate_cards<T: Agent>(
        agent: &mut T,
        deck: &mut Deck,
        top_card: Option<&Card>,
    ) -> std::io::Result<bool> {
        while agent.val() < 21 {
            match agent.action(top_card)? {
                Action::Stay => {
                    TERM.write_line(
                        format!("{}: stayed at {}", agent.name(), agent.val()).as_str(),
                    )?;
                    break;
                }
                Action::Hit => {
                    let card = deck.deal(agent);
                    TERM.write_line(format!("{}: hit and got a {}", agent.name(), card).as_str())?;
                    TERM.write_line(
                        format!("{}: value is {}", agent.name(), agent.val()).as_str(),
                    )?;
                }
            }
        }

        if agent.val() > 21 {
            TERM.write_line(format!("{}: busted with {}", agent.name(), agent.val()).as_str())?;
            Ok(false)
        } else {
            Ok(true)
        }
    }

    fn agent_play<T: Agent>(
        agent: &mut T,
        deck: &mut Deck,
        top_card: Option<&Card>,
    ) -> std::io::Result<bool> {
        if agent.val() == 21 {
            TERM.write_line("Blackjack")?;
            agent.print_hand()?;
            Ok(true)
        } else {
            Game::accumulate_cards(agent, deck, top_card)
        }
    }

    fn player_play(&mut self) -> std::io::Result<bool> {
        let top_card = self.dealer.hand().cards().last().unwrap();
        Game::agent_play(&mut self.player, &mut self.deck, Some(top_card))
    }

    fn dealer_play(&mut self) -> std::io::Result<bool> {
        Game::agent_play(&mut self.dealer, &mut self.deck, None)
    }
}
