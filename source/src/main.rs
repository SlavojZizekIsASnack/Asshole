#[macro_use]
extern crate strum_macros;

use rand::seq::SliceRandom;
use rand::thread_rng;

mod card;
use card::*;

fn main() {
	let mut rng = thread_rng();

	let mut deck = Card::deck();
	deck.shuffle(&mut rng);

	println!("{:#?}", tick_safe(&mut deck));
}

extern "C" {
	fn tick(ptr: *mut Card, len: libc::size_t) -> Card;
}

fn tick_safe(deck: &mut Vec<Card>) -> Card {
	unsafe { tick(deck.as_mut_ptr(), deck.len()) }
}
