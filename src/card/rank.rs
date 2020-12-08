use std::hash::Hash;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Rank {
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
    Ace,
}

impl FromStr for Rank {
    type Err = std::io::Error;

    fn from_str(rank: &str) -> Result<Self, Self::Err> {
        match rank {
            "2" => Ok(Self::Two),
            "3" => Ok(Self::Three),
            "4" => Ok(Self::Four),
            "5" => Ok(Self::Five),
            "6" => Ok(Self::Six),
            "7" => Ok(Self::Seven),
            "8" => Ok(Self::Eight),
            "9" => Ok(Self::Nine),
            "T" => Ok(Self::Ten),
            "J" => Ok(Self::Jack),
            "Q" => Ok(Self::Queen),
            "K" => Ok(Self::King),
            "A" => Ok(Self::Ace),
            _ => panic!("unknown rank value: {}", rank),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::Rank;

    #[test]
    fn test_ordering_two() {
        let lhs = Rank::Two;

        assert!(lhs == Rank::Two);
        assert!(lhs < Rank::Three);
        assert!(lhs < Rank::Four);
        assert!(lhs < Rank::Five);
        assert!(lhs < Rank::Six);
        assert!(lhs < Rank::Seven);
        assert!(lhs < Rank::Eight);
        assert!(lhs < Rank::Nine);
        assert!(lhs < Rank::Ten);
        assert!(lhs < Rank::Jack);
        assert!(lhs < Rank::Queen);
        assert!(lhs < Rank::King);
        assert!(lhs < Rank::Ace);
    }

    #[test]
    fn test_ordering_three() {
        let lhs = Rank::Three;

        assert!(lhs > Rank::Two);
        assert!(lhs == Rank::Three);
        assert!(lhs < Rank::Four);
        assert!(lhs < Rank::Five);
        assert!(lhs < Rank::Six);
        assert!(lhs < Rank::Seven);
        assert!(lhs < Rank::Eight);
        assert!(lhs < Rank::Nine);
        assert!(lhs < Rank::Ten);
        assert!(lhs < Rank::Jack);
        assert!(lhs < Rank::Queen);
        assert!(lhs < Rank::King);
        assert!(lhs < Rank::Ace);
    }

    #[test]
    fn test_ordering_four() {
        let lhs = Rank::Four;

        assert!(lhs > Rank::Two);
        assert!(lhs > Rank::Three);
        assert!(lhs == Rank::Four);
        assert!(lhs < Rank::Five);
        assert!(lhs < Rank::Six);
        assert!(lhs < Rank::Seven);
        assert!(lhs < Rank::Eight);
        assert!(lhs < Rank::Nine);
        assert!(lhs < Rank::Ten);
        assert!(lhs < Rank::Jack);
        assert!(lhs < Rank::Queen);
        assert!(lhs < Rank::King);
        assert!(lhs < Rank::Ace);
    }

    #[test]
    fn test_ordering_five() {
        let lhs = Rank::Five;

        assert!(lhs > Rank::Two);
        assert!(lhs > Rank::Three);
        assert!(lhs > Rank::Four);
        assert!(lhs == Rank::Five);
        assert!(lhs < Rank::Six);
        assert!(lhs < Rank::Seven);
        assert!(lhs < Rank::Eight);
        assert!(lhs < Rank::Nine);
        assert!(lhs < Rank::Ten);
        assert!(lhs < Rank::Jack);
        assert!(lhs < Rank::Queen);
        assert!(lhs < Rank::King);
        assert!(lhs < Rank::Ace);
    }

    #[test]
    fn test_ordering_six() {
        let lhs = Rank::Six;

        assert!(lhs > Rank::Two);
        assert!(lhs > Rank::Three);
        assert!(lhs > Rank::Four);
        assert!(lhs > Rank::Five);
        assert!(lhs == Rank::Six);
        assert!(lhs < Rank::Seven);
        assert!(lhs < Rank::Eight);
        assert!(lhs < Rank::Nine);
        assert!(lhs < Rank::Ten);
        assert!(lhs < Rank::Jack);
        assert!(lhs < Rank::Queen);
        assert!(lhs < Rank::King);
        assert!(lhs < Rank::Ace);
    }

    #[test]
    fn test_ordering_seven() {
        let lhs = Rank::Seven;

        assert!(lhs > Rank::Two);
        assert!(lhs > Rank::Three);
        assert!(lhs > Rank::Four);
        assert!(lhs > Rank::Five);
        assert!(lhs > Rank::Six);
        assert!(lhs == Rank::Seven);
        assert!(lhs < Rank::Eight);
        assert!(lhs < Rank::Nine);
        assert!(lhs < Rank::Ten);
        assert!(lhs < Rank::Jack);
        assert!(lhs < Rank::Queen);
        assert!(lhs < Rank::King);
        assert!(lhs < Rank::Ace);
    }

    #[test]
    fn test_ordering_eight() {
        let lhs = Rank::Eight;

        assert!(lhs > Rank::Two);
        assert!(lhs > Rank::Three);
        assert!(lhs > Rank::Four);
        assert!(lhs > Rank::Five);
        assert!(lhs > Rank::Six);
        assert!(lhs > Rank::Seven);
        assert!(lhs == Rank::Eight);
        assert!(lhs < Rank::Nine);
        assert!(lhs < Rank::Ten);
        assert!(lhs < Rank::Jack);
        assert!(lhs < Rank::Queen);
        assert!(lhs < Rank::King);
        assert!(lhs < Rank::Ace);
    }

    #[test]
    fn test_ordering_nine() {
        let lhs = Rank::Nine;

        assert!(lhs > Rank::Two);
        assert!(lhs > Rank::Three);
        assert!(lhs > Rank::Four);
        assert!(lhs > Rank::Five);
        assert!(lhs > Rank::Six);
        assert!(lhs > Rank::Seven);
        assert!(lhs > Rank::Eight);
        assert!(lhs == Rank::Nine);
        assert!(lhs < Rank::Ten);
        assert!(lhs < Rank::Jack);
        assert!(lhs < Rank::Queen);
        assert!(lhs < Rank::King);
        assert!(lhs < Rank::Ace);
    }

    #[test]
    fn test_ordering_ten() {
        let lhs = Rank::Ten;

        assert!(lhs > Rank::Two);
        assert!(lhs > Rank::Three);
        assert!(lhs > Rank::Four);
        assert!(lhs > Rank::Five);
        assert!(lhs > Rank::Six);
        assert!(lhs > Rank::Seven);
        assert!(lhs > Rank::Eight);
        assert!(lhs > Rank::Nine);
        assert!(lhs == Rank::Ten);
        assert!(lhs < Rank::Jack);
        assert!(lhs < Rank::Queen);
        assert!(lhs < Rank::King);
        assert!(lhs < Rank::Ace);
    }

    #[test]
    fn test_ordering_jack() {
        let lhs = Rank::Jack;

        assert!(lhs > Rank::Two);
        assert!(lhs > Rank::Three);
        assert!(lhs > Rank::Four);
        assert!(lhs > Rank::Five);
        assert!(lhs > Rank::Six);
        assert!(lhs > Rank::Seven);
        assert!(lhs > Rank::Eight);
        assert!(lhs > Rank::Nine);
        assert!(lhs > Rank::Ten);
        assert!(lhs == Rank::Jack);
        assert!(lhs < Rank::Queen);
        assert!(lhs < Rank::King);
        assert!(lhs < Rank::Ace);
    }

    #[test]
    fn test_ordering_queen() {
        let lhs = Rank::Queen;

        assert!(lhs > Rank::Two);
        assert!(lhs > Rank::Three);
        assert!(lhs > Rank::Four);
        assert!(lhs > Rank::Five);
        assert!(lhs > Rank::Six);
        assert!(lhs > Rank::Seven);
        assert!(lhs > Rank::Eight);
        assert!(lhs > Rank::Nine);
        assert!(lhs > Rank::Ten);
        assert!(lhs > Rank::Jack);
        assert!(lhs == Rank::Queen);
        assert!(lhs < Rank::King);
        assert!(lhs < Rank::Ace);
    }

    #[test]
    fn test_ordering_king() {
        let lhs = Rank::King;

        assert!(lhs > Rank::Two);
        assert!(lhs > Rank::Three);
        assert!(lhs > Rank::Four);
        assert!(lhs > Rank::Five);
        assert!(lhs > Rank::Six);
        assert!(lhs > Rank::Seven);
        assert!(lhs > Rank::Eight);
        assert!(lhs > Rank::Nine);
        assert!(lhs > Rank::Ten);
        assert!(lhs > Rank::Jack);
        assert!(lhs > Rank::Queen);
        assert!(lhs == Rank::King);
        assert!(lhs < Rank::Ace);
    }

    #[test]
    fn test_ordering_ace() {
        let lhs = Rank::Ace;

        assert!(lhs > Rank::Two);
        assert!(lhs > Rank::Three);
        assert!(lhs > Rank::Four);
        assert!(lhs > Rank::Five);
        assert!(lhs > Rank::Six);
        assert!(lhs > Rank::Seven);
        assert!(lhs > Rank::Eight);
        assert!(lhs > Rank::Nine);
        assert!(lhs > Rank::Ten);
        assert!(lhs > Rank::Jack);
        assert!(lhs > Rank::Queen);
        assert!(lhs > Rank::King);
        assert!(lhs == Rank::Ace);
    }

    #[test]
    fn test_from_str() {
        assert_eq!(Rank::from_str("2").unwrap(), Rank::Two);
        assert_eq!(Rank::from_str("3").unwrap(), Rank::Three);
        assert_eq!(Rank::from_str("4").unwrap(), Rank::Four);
        assert_eq!(Rank::from_str("5").unwrap(), Rank::Five);
        assert_eq!(Rank::from_str("6").unwrap(), Rank::Six);
        assert_eq!(Rank::from_str("7").unwrap(), Rank::Seven);
        assert_eq!(Rank::from_str("8").unwrap(), Rank::Eight);
        assert_eq!(Rank::from_str("9").unwrap(), Rank::Nine);
        assert_eq!(Rank::from_str("T").unwrap(), Rank::Ten);
        assert_eq!(Rank::from_str("J").unwrap(), Rank::Jack);
        assert_eq!(Rank::from_str("Q").unwrap(), Rank::Queen);
        assert_eq!(Rank::from_str("K").unwrap(), Rank::King);
        assert_eq!(Rank::from_str("A").unwrap(), Rank::Ace);
    }
}
