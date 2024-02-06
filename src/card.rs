use crate::*;

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Copy, Clone)]
pub struct Card {
    pub color: Color,
    pub action: Action,
}

impl Card {
    pub const ACTIONS: [Action; 11] = [
        Action::Zero,
        Action::One,
        Action::Two,
        Action::Three,
        Action::Four,
        Action::Five,
        Action::Six,
        Action::Seven,
        Action::Eight,
        Action::Nine,
        Action::Ten,
    ];

    pub const COLORS: [Color; 4] =
        [Color::Red, Color::Green, Color::Blue, Color::Yellow];

    /// Checks whether self can be played by the argument to this fn
    pub fn is_playable(&self, card_on_deck: Self) -> bool {
        self.action == card_on_deck.action || self.color == card_on_deck.color
    }

    /// For debugging purposes
    pub fn format_deck(deck: Vec<Self>) -> String {
        let mut str = String::new();

        for card in deck.iter() {
            str.push_str(&format!("{card} "))
        }

        str
    }

    pub fn new(color: Color, action: Action) -> Self {
        Self { action, color }
    }

    pub fn make_deck() -> Vec<Self> {
        let mut deck: Vec<Self> = Vec::new();

        Self::COLORS.iter().for_each(|c| {
            Self::ACTIONS
                .iter()
                .for_each(|a| deck.push(Card::new(c.to_owned(), a.to_owned())))
        });

        deck
    }

    pub fn make_random_deck() -> Vec<Self> {
        let mut deck = Self::make_deck();
        deck.shuffle(&mut rand::thread_rng());
        deck
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let action = &self.action;
        let string = match self.color {
            Color::Red => cformat!("<red>[{}]</>", action),
            Color::Green => cformat!("<green>[{}]</>", action),
            Color::Blue => cformat!("<blue>[{}]</>", action),
            Color::Yellow => cformat!("<yellow>[{}]</>", action),
        };

        write!(f, "{string}")
    }
}
