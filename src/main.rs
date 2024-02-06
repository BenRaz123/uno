//! Uno game with card model and self-playing ai

pub mod player;
pub mod ai;
pub mod game;
pub mod card; 
pub mod action;
pub mod color;
pub mod move_result;
pub mod can_play;
pub mod selection;

use color_print::cformat;
use rand::prelude::*;
use std::fmt::{self, Display, Formatter};
use std::{thread::sleep, time::Duration};

use player::Player;
use ai::Ai;
use game::Game;
use card::Card;
use action::Action;
use color::Color;
use can_play::CanPlay;
use move_result::MoveResult;

fn main() {
    let mut game = Game::new();
    let mut player = Player::new(&mut game);
    let mut ai = Ai::new(&mut game);
    let mut last_result = MoveResult::None;
    
    while let MoveResult::None = last_result {
        last_result = player.play_round(&mut game, ai.get_card_count(), &last_result);
        last_result = ai.play_round(&mut game, player.get_card_count(), &last_result); 
    }

    println!("Ai Won!");
}
