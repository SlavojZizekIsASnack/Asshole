#[macro_use]
extern crate strum_macros;

mod card;
use card::*;

fn main() {
	let c = Card::new(Face::Queen, Suit::Hearts);

	println!("{}", unsafe { tick(10) });
}

extern "C" {
	fn tick(a: libc::c_int) -> libc::c_int;
}
