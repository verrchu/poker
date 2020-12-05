use super::Rank;

#[derive(Debug, Clone, PartialEq)]
pub enum Combination {
    HighCard { rank: Rank },
    Pair { rank: Rank, kicker: Rank },
    TwoPairs { low: Rank, high: Rank, kicker: Rank },
    ThreeOfAKind { rank: Rank, kicker: Rank },
    Straight { low: Rank, high: Rank },
    Flush { high: Rank },
    FullHouse { two: Rank, three: Rank },
    FourOfAKind { rank: Rank, kicker: Rank },
    StraightFlush { low: Rank, high: Rank },
}
