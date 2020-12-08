use std::hash::Hash;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Suite {
    Diamonds,
    Clubs,
    Hearts,
    Spades,
}

impl FromStr for Suite {
    type Err = std::io::Error;

    fn from_str(suite: &str) -> Result<Self, Self::Err> {
        match suite {
            "c" => Ok(Self::Clubs),
            "d" => Ok(Self::Diamonds),
            "h" => Ok(Self::Hearts),
            "s" => Ok(Self::Spades),
            _ => panic!("unknown suite value: {}", suite),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Suite;

    #[test]
    fn test_equality() {
        assert!(Suite::Diamonds == Suite::Diamonds);
        assert!(Suite::Clubs == Suite::Clubs);
        assert!(Suite::Hearts == Suite::Hearts);
        assert!(Suite::Spades == Suite::Spades);
    }
}
