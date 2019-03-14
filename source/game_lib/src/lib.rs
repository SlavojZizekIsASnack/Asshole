#[macro_use]
extern crate strum_macros;

pub mod card;
use card::*;

pub trait Player {
    fn play(hand: Vec<Card>, play_type: PlayType, last_card: Card) -> Card;
    fn new() -> Self;
}
