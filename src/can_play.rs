use crate::{Card, Game, MoveResult, selection::Selection};

pub trait CanPlay {
    fn get_deck(&mut self) -> &mut Vec<Card>;

    fn get_card_count(&self) -> u8;

    fn play_card_index(&mut self, game: &mut Game, index: usize) {
        let deck = self.get_deck();
        let card = deck[index];
        deck.remove(index);
        game.play_card(card);
    }
    
    fn play_round(
        &mut self,
        game: &mut Game,
        opponent_card_count: u8,
        last_move_result: &MoveResult,
    ) -> MoveResult;
}
