use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::{SystemTime, UNIX_EPOCH};

// To be able to check equality
#[derive(PartialEq, Debug)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

#[derive(PartialEq, Debug)]
pub enum Rank {
    Ace,
    Number(u8),
    Jack,
    Queen,
    King,
}

// Simple random number generator without external dependencies
struct SimpleRng {
    seed: u64,
}

impl SimpleRng {
    fn new() -> Self {
        let mut hasher = DefaultHasher::new();
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
            .hash(&mut hasher);
        SimpleRng {
            seed: hasher.finish(),
        }
    }

    fn next(&mut self) -> u64 {
        // Linear congruential generator
        self.seed = self.seed.wrapping_mul(1103515245).wrapping_add(12345);
        self.seed
    }

    fn random_range(&mut self, range: std::ops::Range<u8>) -> u8 {
        let range_size = range.end - range.start;
        (self.next() % range_size as u64) as u8 + range.start
    }
}

impl Suit {
    pub fn random() -> Suit {
        let mut rng = SimpleRng::new();
        // Return a suit depending on the random number generated
        match rng.random_range(0..4) {
            0 => Suit::Heart,
            1 => Suit::Diamond,
            2 => Suit::Spade,
            3 => Suit::Club,
            _ => unreachable!(),
        }
    }

    // Convert an integer to a suit
    pub fn translate(value: u8) -> Suit {
        match value {
            1 => Suit::Heart,
            2 => Suit::Diamond,
            3 => Suit::Spade,
            4 => Suit::Club,
            _ => unreachable!(),
        }
    }
}

impl Rank {
    pub fn random() -> Rank {
        let mut rng = SimpleRng::new();
        let number = rng.random_range(1..14); // 1-13 for standard deck
        // Return a rank depending on the random number generated
        match number {
            1 => Rank::Ace,
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            number => Rank::Number(number),
        }
    }

    pub fn translate(value: u8) -> Rank {
        match value {
            1 => Rank::Ace,
            2..=10 => Rank::Number(value),
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            _ => unreachable!(),
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: &Card) -> bool {
    card.suit == Suit::Spade && card.rank == Rank::Ace
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_winner_card() {
        let ace_of_spades = Card {
            suit: Suit::Spade,
            rank: Rank::Ace,
        };
        assert!(winner_card(&ace_of_spades));

        let ace_of_hearts = Card {
            suit: Suit::Heart,
            rank: Rank::Ace,
        };
        assert!(!winner_card(&ace_of_hearts));
    }

    #[test]
    fn test_suit_translate() {
        assert_eq!(Suit::translate(1), Suit::Heart);
        assert_eq!(Suit::translate(2), Suit::Diamond);
        assert_eq!(Suit::translate(3), Suit::Spade);
        assert_eq!(Suit::translate(4), Suit::Club);
    }

    #[test]
    fn test_rank_translate() {
        assert_eq!(Rank::translate(1), Rank::Ace);
        assert_eq!(Rank::translate(5), Rank::Number(5));
        assert_eq!(Rank::translate(11), Rank::Jack);
        assert_eq!(Rank::translate(12), Rank::Queen);
        assert_eq!(Rank::translate(13), Rank::King);
    }
}
