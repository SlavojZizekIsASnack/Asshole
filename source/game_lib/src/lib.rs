#[macro_use]
extern crate strum_macros;

pub mod card;
use card::*;

pub trait Player {
	fn tick(&mut self, play_type: PlayType, last_card: Card) -> Card;
	fn set_hand(&mut self, hand: Vec<Card>);
	fn get_mut_hand(&mut self) -> &mut Vec<Card>;
	fn get_hand(&self) -> &Vec<Card>;
}

pub struct Game {
	pub players: Vec<Box<Player>>,
	pub pile: Vec<Card>,
}

impl Game {
	// Creates a Game, create and shuffle deck, and split deck off.
	pub fn new(mut players: Vec<Box<Player>>) -> Self {
		// Creates a shuffled deck.
		let mut deck = Card::shuffled_deck();

		// Size of player decks
		let player_deck_size = deck.len() / players.len();

		// Give player their deck and sort their deck.
		for player in &mut players {
			let mut deck = deck.split_off(deck.len() - player_deck_size);
			deck.sort();
			player.set_hand(deck);
		}

		// Return game struct.
		Game {
			players,
			pile: vec![],
		}
	}

	pub fn tick(&mut self) -> Result<(), TickError> {
		for (i, player) in self.players.iter_mut().enumerate() {
			let play = player.tick(PlayType::Clear, Card::new(Face::Three, Suit::Diamonds));

			println!("Player {} played '{}'", i, play)
		}
		println!("One turn played");

		Ok(())
	}
}

pub enum TickError {
	IllegalMove,
	NoneExistingCard,
}
