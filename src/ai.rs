//! Module for the [`Ai`] struct and it's implementations.

use crate::{Game, Card, MoveResult, can_play::CanPlay}; 

/// The in game "AI" (no actual AI models used). Owns a "deck" of cards (`Vec<[Card]>`) 
pub struct Ai {
    pub deck: Vec<Card>
}

impl CanPlay for Ai {
    fn get_deck(&mut self) -> &mut Vec<Card> {
        &mut self.deck
    }

    fn get_card_count(&self) -> u8 {
        self.deck.len() as u8
    }

    fn play_round(
            &mut self,
            game: &mut Game,
            _opponent_card_count: u8,
            last_move_result: &MoveResult,
        ) -> MoveResult {

        self.deck.sort();

        if let MoveResult::Won = last_move_result {
            return MoveResult::None;
        }

        if game.get_top_card().is_none() {
            self.play_card_index(game, 0)
        }

        for (index,card) in self.deck.iter().enumerate() {
            if !card.is_playable(game.get_top_card().unwrap()) { continue; }
            game.play_card(card.to_owned());
            self.deck.remove(index);
            if self.deck.is_empty() { return MoveResult::Won; }
            return MoveResult::None;
        }

        game.draw_a_card(self);
        MoveResult::None

    }
}

impl Ai {
    pub fn new(game: &mut Game) -> Self {
        let mut ai = Self {deck: vec![] };
        for _ in 0..7 {
            game.draw_a_card(&mut ai);
        }
        ai
    }
}
