use crate::Card;

use color_print::cformat;
use console::{Term, pad_str};

use std::fmt::{self, Display, Formatter};
use std::ops::{AddAssign, SubAssign};

pub enum SelectionItem {
    Card(Card),
    Draw,
    Quit,
}

pub struct Selection {
    pub options: Vec<SelectionItem>,
    pub index: usize,
}

impl Display for Selection {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut str = String::new();
        for (index, option) in self.options.iter().enumerate() {
            if index == self.index {
                str.push_str(&cformat!("<bold><u>{option}</></> "));
                continue;
            }
            str.push_str(&format!("{option} "));
        }
        write!(f, "{str}")
    }
}

impl Display for SelectionItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let str: String = match self {
            Self::Card(card) => card.to_string(),
            Self::Draw => String::from("Draw a Card"),
            Self::Quit => String::from("Quit Game (Irreversible)"),
        };
        write!(f, "{str}")
    }
}

impl SelectionItem {
    fn from_card(card: Card) -> Self {
        Self::Card(card)
    }
}

impl AddAssign<i32> for Selection {
    fn add_assign(&mut self, _: i32) {
        let max = self.options.len() - 1;
        if self.index == max {
            self.index = 0;
            return;
        }
        self.index += 1;
    }
}

impl SubAssign<i32> for Selection {
    fn sub_assign(&mut self, _: i32) {
        let max = self.options.len() - 1;
        if self.index == 0 {
            self.index = max;
            return;
        }
        self.index -= 1;
    }
}

impl Selection {
    pub fn new(options: Vec<SelectionItem>) -> Self {
        Self { options, index: 0 }
    }
    pub fn from_deck(deck: Vec<Card>) -> Selection {
        let mut selection = Self::new(vec![]);
        deck.iter().for_each(|card| {
            selection
                .options
                .push(SelectionItem::from_card(card.to_owned()))
        });
        selection.options.push(SelectionItem::Draw);
        selection.options.push(SelectionItem::Quit);
        selection
    }
    
    pub fn update_selection_display(&self, top_card: Option<Card>) {

        let card = top_card.clone();

        let preceding = match &card {
            None => "(No Card)".into(),
            Some(card) => format!("(Deck: {card})"),
        };
        
        let string = center_string(&format!("{preceding} {self}"));        

        print!("{}[2J", 27 as char);
        println!("{string}");
    }
    
    pub fn prompt(&mut self, top_card: Option<Card>) {
        let stdout = console::Term::buffered_stdout();
        self.update_selection_display(top_card);
        loop {
            if let Ok(c) = stdout.read_char() {
                match c {
                    'a' | 'A' | 'h' | 'H' => *self -= 1,
                    'd' | 'D' | 'l' | 'L' => *self += 1,
                    '\n' => return,
                    _ => (),
                }
                self.update_selection_display(top_card);
            }
        }
    }
}

fn center_string(s: &str) -> String {
    let width = Term::stdout().size().1;
    let height = Term::stdout().size().0;

    pad_str(s, width as usize, console::Alignment::Center, None).into()
}
