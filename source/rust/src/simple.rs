use game_lib::card::*;
use game_lib::Player;

pub struct Simple;

impl Player for Simple {
	fn new() -> Simple {
		Simple {}
	}

	fn play(hand: Vec<Card>, play_type: PlayType, pile: Vec<Card>) -> Card {
		hand[0]
	}
}
