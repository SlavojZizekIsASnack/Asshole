use game_lib::card::*;
use game_lib::Player;

pub struct Simple;

extern "C" {
	fn play_simple(
		deck_ptr: *mut Card,
		deck_len: libc::size_t,
		play_type: PlayType,
		pile_ptr: *mut Card,
		pile_len: libc::size_t,
	) -> Card;
}

impl Player for Simple {
	fn play(mut deck: Vec<Card>, play_type: PlayType, mut pile: Vec<Card>) -> Card {
		unsafe {
			play_simple(
				deck.as_mut_ptr(),
				deck.len(),
				play_type,
				pile.as_mut_ptr(),
				pile.len(),
			)
		}
	}

	fn new() -> Self {
		Simple {}
	}
}
