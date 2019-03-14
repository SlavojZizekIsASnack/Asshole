use rand::seq::SliceRandom;
use rand::thread_rng;

use game_lib::card::*;

fn main() {
	init_game(4);
}

fn init_game(players: usize) -> Vec<Vec<Card>> {
	let mut deck = Card::deck();
	let mut rng = thread_rng();
	// Amount of cards in a players hand, at the start
	// of the game.
	let player_hand_size = deck.len() / players;

	deck.shuffle(&mut rng);

	vec![deck]
}
