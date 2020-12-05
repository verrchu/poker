pub mod card;
pub mod combination;
pub mod game;

pub use card::Card;
pub use card::Rank;
pub use card::Suite;

pub use game::Board;
pub use game::Game;
pub use game::HandOf2;
pub use game::HandOf4;
pub use game::HandOf5;

#[cfg(test)]
mod tests {
    use super::Rank;

    #[test]
    fn test_card_rank_ordering() {
        assert!(Rank::Three > Rank::Two);
        assert!(Rank::Four > Rank::Three);
        assert!(Rank::Five > Rank::Four);
        assert!(Rank::Six > Rank::Five);
        assert!(Rank::Seven > Rank::Six);
        assert!(Rank::Eight > Rank::Seven);
        assert!(Rank::Nine > Rank::Eight);
        assert!(Rank::Ten > Rank::Nine);
        assert!(Rank::Jack > Rank::Ten);
        assert!(Rank::Queen > Rank::Jack);
        assert!(Rank::King > Rank::Queen);
        assert!(Rank::Ace > Rank::King);
    }

    #[test]
    fn test_card_rank_equality() {
        assert!(Rank::Three == Rank::Three);
        assert!(Rank::Four == Rank::Four);
        assert!(Rank::Five == Rank::Five);
        assert!(Rank::Six == Rank::Six);
        assert!(Rank::Seven == Rank::Seven);
        assert!(Rank::Eight == Rank::Eight);
        assert!(Rank::Nine == Rank::Nine);
        assert!(Rank::Ten == Rank::Ten);
        assert!(Rank::Jack == Rank::Jack);
        assert!(Rank::Queen == Rank::Queen);
        assert!(Rank::King == Rank::King);
        assert!(Rank::Ace == Rank::Ace);
    }
}
