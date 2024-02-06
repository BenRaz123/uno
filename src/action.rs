//! Module for the [`Action`] struct and it's associated implementations

use crate::*;

#[derive(PartialOrd, PartialEq, Eq, Ord, Debug, Copy, Clone)]
/// An action that can be on a card.
pub enum Action {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
}

impl Action {
    /// Convert the action to a `u8` if it is numeric, otherwise return `None`
    pub fn try_to_u8(&self) -> u8 {
        match self {
            Self::Zero => 0,
            Self::One => 1,
            Self::Two => 2,
            Self::Three => 3,
            Self::Four => 4,
            Self::Five => 5,
            Self::Six => 6,
            Self::Seven => 7,
            Self::Eight => 8,
            Self::Nine => 9,
            Self::Ten => 10,
        }
    }
}


impl Display for Action {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.try_to_u8())
    }
}
