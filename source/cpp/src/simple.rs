use game_lib::card::*;

extern "C" {
	fn play_unsafe(
		deck_ptr: *mut Card,
		deck_len: libc::size_t,
		play_type: PlayType,
		last_card: Card,
	) -> Card;
}

pub fn play(deck: &mut Vec<Card>, play_type: PlayType, last_card: Card) -> Card {
	unsafe { play_unsafe(deck.as_mut_ptr(), deck.len(), play_type, last_card) }
}