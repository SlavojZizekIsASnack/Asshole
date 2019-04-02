#[macro_use]
extern crate strum_macros;

use rand::seq::SliceRandom;
use rand::thread_rng;

pub mod card;
use card::*;

pub struct Player {
	pub deck: Vec<Card>,
	pub tick: fn(&mut Vec<Card>, PlayType, Card) -> Card,
}

impl Player {
	fn new(deck: Vec<Card>, tick: fn(&mut Vec<Card>, PlayType, Card) -> Card) -> Self {
		Player { deck, tick }
	}
}

pub struct Game {
	pub players: Vec<Player>,
	pub pile: Vec<Card>,
}

impl Game {
	// Creates a Game, create and shuffle deck, and split deck off.
	pub fn new(ticks: Vec<fn(&mut Vec<Card>, PlayType, Card) -> Card>) -> Self {
		let mut deck = Game::deck();
		let mut players = vec![];

		// Size of player decks
		let player_deck_size = deck.len() / ticks.len();

		for tick in ticks {
			players.push(Player::new(
				deck.split_off(deck.len() - player_deck_size),
				tick,
			));

			// Garanteret ikke at crashe, fordi der lige
			// er tilfoejet en player til vektoren
			players.last_mut().unwrap().deck.sort();
		}

		Game {
			players,
			pile: vec![],
		}
	}

	// Returns a Vec of shuffled cards.
	fn deck() -> Vec<Card> {
		let mut deck = Card::deck();
		let mut rng = thread_rng();
		deck.shuffle(&mut rng);

		deck
	}

	// Iteraters
	pub fn tick(&mut self) {
		for (i, player) in self.players.iter_mut().enumerate() {
			let play = (player.tick)(
				&mut player.deck,
				PlayType::Clear,
				Card::new(Face::Ace, Suit::Clubs),
			);

			println!("Player {} played '{}'", i, play)
		}
		println!("One turn played");
	}

	fn is_legal_move() -> bool {
		true
	}
}
