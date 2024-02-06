//! Uno game with card model and self-playing ai

pub mod action;
pub mod ai;
pub mod can_play;
pub mod card;
pub mod color;
pub mod game;
pub mod move_result;
pub mod player;
pub mod selection;

use color_print::cformat;
use rand::prelude::*;
use std::fmt::{self, Display, Formatter};
use std::{thread::sleep, time::Duration};

use action::Action;
use ai::Ai;
use can_play::CanPlay;
use card::Card;
use color::Color;
use game::Game;
use move_result::MoveResult;
use player::Player;

fn main() {
    let mut game = Game::new();
    let mut player = Player::new(&mut game);
    let mut ai = Ai::new(&mut game);
    let mut last_result = MoveResult::None;

    while let MoveResult::None = last_result {
        last_result =
            player.play_round(&mut game, ai.get_card_count(), &last_result);
        last_result =
            ai.play_round(&mut game, player.get_card_count(), &last_result);
    }

    println!("Ai Won!");
}
