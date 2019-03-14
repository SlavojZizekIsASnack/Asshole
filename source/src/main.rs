use rand::seq::SliceRandom;
use rand::thread_rng;

use game_lib::card::*;
use game_lib::Player;

fn main() {
	init_game();
}

fn init_game(players: Vec<(Box<impl Player>, Vec<Card>)>) -> Vec<Vec<Card>> {
	let mut deck = Card::deck();
	let mut rng = thread_rng();
	let mut hands = vec![];
	// Amount of cards in a players hand, at the start
	// of the game.
	let player_hand_size = deck.len() / players.len();

	deck.shuffle(&mut rng);

	for i in players {}

	hands
}
