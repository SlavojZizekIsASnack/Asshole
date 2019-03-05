#[macro_use]
extern crate strum_macros;

mod card;
use card::*;

fn main() {
	let c = Card::new(Face::Queen, Suit::Hearts);

	println!("{}", c);
}

extern "C" {
	fn play(ss: Vec<*const Card>) -> libc::c_int;

	fn test(ss: libc::c_int) -> libc::c_int;
}
f