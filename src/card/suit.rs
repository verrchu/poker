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

impl ToString for Suit {
    fn to_string(&self) -> String {
        match self {
            Self::Clubs => "c".to_string(),
            Self::Diamonds => "d".to_string(),
            Self::Hearts => "h".to_string(),
            Self::Spades => "s".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::Suit;

    #[test]
    fn test_equality() {
        assert_eq!(Suit::Diamonds, Suit::Diamonds);
        assert_eq!(Suit::Clubs, Suit::Clubs);
        assert_eq!(Suit::Hearts, Suit::Hearts);
        assert_eq!(Suit::Spades, Suit::Spades);
    }

    #[test]
    fn test_from_str() {
        assert_eq!(Suit::from_str("c").unwrap(), Suit::Clubs);
        assert_eq!(Suit::from_str("d").unwrap(), Suit::Diamonds);
        assert_eq!(Suit::from_str("h").unwrap(), Suit::Hearts);
        assert_eq!(Suit::from_str("s").unwrap(), Suit::Spades);
    }

    #[test]
    fn test_to_string() {
        assert_eq!(Suit::Clubs.to_string(), "c".to_string());
        assert_eq!(Suit::Diamonds.to_string(), "d".to_string());
        assert_eq!(Suit::Hearts.to_string(), "h".to_string());
        assert_eq!(Suit::Spades.to_string(), "s".to_string());
    }
}
