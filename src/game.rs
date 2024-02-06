use crate::{Card, CanPlay};

pub struct Game {
    pub draw: Vec<Card>,
    pub discard: Vec<Card>,
}

impl Game {
    pub fn new() -> Self {
        let draw = Card::make_random_deck();
        let discard = Vec::new();
        Self { draw, discard }
    }

    pub fn draw_a_card<P: CanPlay>(&mut self, player: &mut P){
        if self.draw.is_empty() {
            self.discard.reverse();
            self.draw = self.discard.to_owned();
            self.discard = Vec::new();
        }

        let draw_card = self.draw.pop().unwrap();
        player.get_deck().push(draw_card)
    }

    pub fn play_card(&mut self, card: Card) {
        self.discard.push(card)
    }

    pub fn get_top_card(&self) -> Option<Card> {
        if self.discard.is_empty() {
            return None;
        }
        let len = self.discard.len() - 1;
        Some(self.discard[len])
    }
}
