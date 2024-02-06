use std::process::exit;

use crate::{
    can_play::CanPlay,
    card::Card,
    game::Game,
    move_result::MoveResult,
    selection::{Selection, SelectionItem},
};
use console;

pub struct Player {
    deck: Vec<Card>,
}
impl Player {
    pub fn new(game: &mut Game) -> Self {
        let mut player = Player { deck: vec![] };
        game.draw_a_card(&mut player);
        game.draw_a_card(&mut player);
        game.draw_a_card(&mut player);
        game.draw_a_card(&mut player);
        game.draw_a_card(&mut player);
        game.draw_a_card(&mut player);
        game.draw_a_card(&mut player);
        player
    }
}
impl CanPlay for Player {
    fn get_deck(&mut self) -> &mut Vec<Card> {
        &mut self.deck
    }

    fn get_card_count(&self) -> u8 {
        self.deck.len() as u8
    }

    fn play_round(
        &mut self,
        game: &mut Game,
        opponent_card_count: u8,
        _last_move_result: &MoveResult,
    ) -> MoveResult {
        let mut options = Selection::from_deck(self.deck.to_owned());
        loop {
            options.prompt(game.get_top_card(), opponent_card_count);
            let selected = &options.options[options.index];
            match selected {
                SelectionItem::Draw => {
                    game.draw_a_card(self);
                    break;
                }
                SelectionItem::Quit => exit(0),
                SelectionItem::Card(card) => match game.get_top_card() {
                    Some(top_card) => match card.is_playable(top_card) {
                        true => {
                            let index = options.index;
                            self.play_card_index(game, index);
                            break;
                        }
                        false => (),
                    },
                    None => {
                        let index = options.index;
                        self.play_card_index(game, index);
                        break;
                    }
                },
            }
        }
        if self.deck.is_empty() {
            println!("You Won!");
            exit(0);
        }
        MoveResult::None
    }
}
