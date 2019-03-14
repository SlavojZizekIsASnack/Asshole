use game_lib::card::*;
use game_lib::Player;

pub struct Simple;

impl Player for Simple {
	fn new() -> Self {
		Simple {}
	}

	fn play(hand: Vec<Card>, play_type: PlayType, last_card: Card) -> Card {
		hand[0]
	}
}
