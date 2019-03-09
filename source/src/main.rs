#[macro_use]
extern crate strum_macros;

use rand::seq::SliceRandom;
use rand::thread_rng;

mod card;
use card::*;

fn main() {
	let mut rng = thread_rng();

	let d = Card::deck().shuffle(&mut rng);

	println!("{:#?}", Card::deck());
}

extern "C" {
	fn find_suit(a: Card) -> Suit;
}
