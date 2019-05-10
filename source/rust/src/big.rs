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
			// This bot always plays single cards and avoids playing combos.
			PlayType::Single => {
				// Safe because we're guaranteed atleast one card
				if self.hand.last().unwrap() >= &last_card {
					vec![(self.hand.len() - 1) as u32]
				} else {
					// We don't have a big enough card.
					vec![]
				}
			}
			PlayType::Clear | PlayType::Double => {
				match self
					.hand
					.iter()
					// Add index to iterator
					.enumerate()
					// Clone hand iter, so we can window it
					.collect::<Vec<(usize, &Card)>>()
					// Divide into overlapping chunks of two
					.windows(2)
					// Reverse because we want to find
					.rev()
					// Find a sequence where all that cards are the same
					// and are the same or higher as the last_card.
					.find(|item| {
						item.iter().min().unwrap().1 == item.iter().max().unwrap().1
							&& item[0].1 >= &last_card
					}) {
					Some(cards) => {
						let mut vec = vec![];
						for (i, _) in cards {
							vec.push(*i as u32)
						}

						vec
					}
					None => vec![],
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