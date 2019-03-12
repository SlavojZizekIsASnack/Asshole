#[macro_use]
extern crate strum_macros;

mod card;
use card::*;

fn main() {
	use rand::seq::SliceRandom;
	use rand::thread_rng;
	let mut rng = thread_rng();

	let mut deck = Card::deck();
	deck.shuffle(&mut rng);
	deck.sort();

	println!("{:#?}", tick_safe(deck));
}

extern "C" {
	fn tick(ptr: *mut Card, len: libc::size_t) -> Card;
}

fn tick_safe(mut deck: Vec<Card>) -> Card {
	unsafe { tick(deck.as_mut_ptr(), deck.len()) }
}

trait Player {
	fn play(hand: Vec<Card>, play_type: PlayType, pile: Vec<Card>) -> Card;
}
