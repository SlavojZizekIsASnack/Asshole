use game_lib::card::*;
use game_lib::Player;

pub struct Big {
	hand: Vec<Card>,
	name: String,
}

impl Big {
	pub fn new(name: impl Into<String>) -> Box<Player> {
		Box::new(Self {
			hand: vec![],
			name: name.into(),
		})
	}
}

impl Player for Big {
	fn tick(&mut self, play_type: PlayType, last_card: Card) -> Vec<u32> {
		match play_type {
			// Largest item must be the biggest. Select it
			PlayType::Clear | PlayType::Single => {
				// Safe because we're guaranteed atleast one card
				if self.hand.last().unwrap() >= &last_card {
					vec![(self.hand.len() - 1) as u32]
				} else {
					// We don't have a big enough card.
					vec![]
				}
			}
			_ => panic!(),
		}
	}

	fn set_hand(&mut self, hand: Vec<Card>) {
		self.hand = hand;
	}

	fn get_mut_hand(&mut self) -> &mut Vec<Card> {
		&mut self.hand
	}

	fn get_hand(&self) -> &Vec<Card> {
		&self.hand
	}

	fn get_name(&self) -> &String {
		&self.name
	}
}