#[macro_use]
extern crate strum_macros;

mod card;
use card::*;

fn main() {
	let c = Card::new(Face::Queen, Suit::Hearts);

	println!("{}", unsafe { test(2) });
}

extern "C" {
	fn test(a: libc::c_int) -> libc::c_int;
}
