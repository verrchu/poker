use std::hash::Hash;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Suit {
    Diamonds,
    Clubs,
    Hearts,
    Spades,
}

impl FromStr for Suit {
    type Err = std::io::Error;

    fn from_str(suit: &str) -> Result<Self, Self::Err> {
        match suit {
            "c" => Ok(Self::Clubs),
            "d" => Ok(Self::Diamonds),
            "h" => Ok(Self::Hearts),
            "s" => Ok(Self::Spades),
            _ => panic!("unknown suit value: {}", suit),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::Suit;

    #[test]
    fn test_equality() {
        assert!(Suit::Diamonds == Suit::Diamonds);
        assert!(Suit::Clubs == Suit::Clubs);
        assert!(Suit::Hearts == Suit::Hearts);
        assert!(Suit::Spades == Suit::Spades);
    }

    #[test]
    fn test_from_str() {
        assert_eq!(Suit::from_str("c").unwrap(), Suit::Clubs);
        assert_eq!(Suit::from_str("d").unwrap(), Suit::Diamonds);
        assert_eq!(Suit::from_str("h").unwrap(), Suit::Hearts);
        assert_eq!(Suit::from_str("s").unwrap(), Suit::Spades);
    }
}
