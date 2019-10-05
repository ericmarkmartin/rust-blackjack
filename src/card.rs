use itertools::iproduct;
use std::convert::TryFrom;
use std::{cmp, fmt};

lazy_static::lazy_static! {
    pub static ref ALL_CARDS: Vec<Card> = iproduct!(Suit::all().iter(), Rank::all().iter())
        .map(|(suit, rank)| Card::new(*rank, *suit))
        .collect();
}

fn div_ceil(x: usize, y: usize) -> usize {
    (x - 1) / y + 1
}

#[derive(Copy, Clone)]
pub struct Card {
    rank: Rank,
    suit: Suit,
}

impl Card {
    fn new(rank: Rank, suit: Suit) -> Self {
        Card { rank, suit }
    }

    fn initial_val(&self) -> usize {
        self.rank.initial_val()
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.rank, self.suit)
    }
}

pub struct Hand(Vec<Card>);

impl Hand {
    pub fn new() -> Self {
        Hand(Vec::new())
    }

    pub fn val(&self) -> usize {
        let mut num_aces = 0;
        let sum = self
            .0
            .iter()
            .map(|card| {
                if let Rank::Ace = card.rank {
                    num_aces += 1;
                }
                card.initial_val()
            })
            .sum();

        if sum > 21 {
            // 11 - 1 = 10 gives decrease from changing ace from 11 to 1
            let num_aces_needed = div_ceil(sum - 21, 10);
            sum - 10 * cmp::min(num_aces_needed, num_aces)
        } else {
            sum
        }
    }

    pub fn add(&mut self, card: Card) {
        self.0.push(card);
    }

    pub fn cards(&self) -> &Vec<Card> {
        &self.0
    }

    pub fn mut_cards(&mut self) -> &mut Vec<Card> {
        &mut self.0
    }

    fn cards_string<'a, T: Iterator<Item = &'a Card>>(cards: T) -> String {
        cards
            .map(|card| format!("{}", card))
            .collect::<Vec<_>>()
            .join(" ")
    }

    pub fn hand_string(&self) -> String {
        Hand::cards_string(self.0.iter())
    }

    pub fn hidden_hand_string(&self) -> String {
        Hand::cards_string(self.0.iter().skip(1))
    }
}

#[derive(Copy, Clone, Debug)]
enum Rank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

impl Rank {
    pub fn all() -> [Rank; 13] {
        use Rank::*;
        [
            Ace, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King,
        ]
    }

    fn initial_val(&self) -> usize {
        use Rank::*;
        match self {
            Ace => 11,
            Two => 2,
            Three => 3,
            Four => 4,
            Five => 5,
            Six => 6,
            Seven => 7,
            Eight => 8,
            Nine => 9,
            Ten => 10,
            Jack => 10,
            Queen => 10,
            King => 10,
        }
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Rank::*;
        let s = match self {
            Ace => "A",
            Two => "2",
            Three => "3",
            Four => "4",
            Five => "5",
            Six => "6",
            Seven => "7",
            Eight => "8",
            Nine => "9",
            Ten => "10",
            Jack => "J",
            Queen => "Q",
            King => "K",
        };
        write!(f, "{}", s)
    }
}

impl TryFrom<usize> for Rank {
    type Error = &'static str;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        use Rank::*;
        match value {
            0 => Err("No card for zero"),
            1 => Ok(Ace),
            2 => Ok(Two),
            3 => Ok(Three),
            4 => Ok(Four),
            5 => Ok(Five),
            6 => Ok(Six),
            7 => Ok(Seven),
            8 => Ok(Eight),
            9 => Ok(Nine),
            10 => Ok(Ten),
            11 => Ok(Jack),
            12 => Ok(Queen),
            13 => Ok(King),
            _ => Err("No card for value greater than thirteen"),
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Suit {
    Spades,
    Hearts,
    Clubs,
    Diamonds,
}

impl Suit {
    fn all() -> [Suit; 4] {
        use Suit::*;
        [Spades, Hearts, Clubs, Diamonds]
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Suit::*;
        let s = match self {
            Spades => "♠",
            Hearts => "♥",
            Diamonds => "♦",
            Clubs => "♣",
        };
        write!(f, "{}", s)
    }
}

// impl TryFrom<usize> for Suite {
//     type Error = &'static str;
//     fn try_from(value: usize) -> Result<Self, Self::Error> {
//         use Rank::*;
//         match value {
//             0 => Spades,
//             1 => Hearts,
//             2 => Clubs,
//             3 => Diamonds
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_normal_hand_value() {
        let hand = Hand(vec![
            Card::new(Rank::Seven, Suit::Hearts),
            Card::new(Rank::Four, Suit::Hearts),
        ]);

        assert_eq!(hand.val(), 11);
    }

    #[test]
    fn test_blackjack_hand_value() {
        let hand = Hand(vec![
            Card::new(Rank::King, Suit::Diamonds),
            Card::new(Rank::Ace, Suit::Spades),
        ]);

        assert_eq!(hand.val(), 21);
    }

    #[test]
    fn test_adjust_single_ace_hand_value() {
        let hand = Hand(vec![
            Card::new(Rank::Ten, Suit::Clubs),
            Card::new(Rank::Eight, Suit::Diamonds),
            Card::new(Rank::Ace, Suit::Clubs),
        ]);

        assert_eq!(hand.val(), 19);
    }

    #[test]
    fn test_adjust_multi_ace_hand_value() {
        let hand = Hand(vec![
            Card::new(Rank::Ten, Suit::Clubs),
            Card::new(Rank::Eight, Suit::Diamonds),
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Ace, Suit::Diamonds),
        ]);

        assert_eq!(hand.val(), 20);
    }

    #[test]
    fn test_dont_overadjust_ace_hand_value() {
        let hand = Hand(vec![
            Card::new(Rank::Ten, Suit::Spades),
            Card::new(Rank::Seven, Suit::Spades),
            Card::new(Rank::Three, Suit::Clubs),
            Card::new(Rank::Ace, Suit::Hearts),
            Card::new(Rank::Ace, Suit::Diamonds),
        ]);

        assert_eq!(hand.val(), 22);
    }

    #[test]
    fn test_bust_hand_value() {
        let hand = Hand(vec![
            Card::new(Rank::Ten, Suit::Clubs),
            Card::new(Rank::Eight, Suit::Spades),
            Card::new(Rank::Six, Suit::Diamonds),
        ]);

        assert_eq!(hand.val(), 24);
    }
}
