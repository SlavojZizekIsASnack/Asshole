use rand::seq::SliceRandom;
use rand::thread_rng;

use game_lib::card::*;
use game_lib::Player;

fn main() {
	let mut deck = Card::deck();
	let mut rng = thread_rng();
	deck.shuffle(&mut rng);
}
