#[macro_use]
extern crate strum_macros;

mod card;
use card::*;

fn main() {
	let c = Card::new(Face::Queen, Suit::Hearts);

	println!("{}", unsafe { tick(c) });
}

extern "C" {
	fn tick(a: Card) -> Suit;
}
