use game_lib::card::*;
use game_lib::Player;

pub struct BigCombo {
	hand: Vec<Card>,
	name: String,
}

impl BigCombo {
	pub fn new(name: impl Into<String>) -> Box<Player> {
		Box::new(Self {
			hand: vec![],
			name: name.into(),
		})
	}
}

impl Player for BigCombo {
	fn tick(&mut self, play_type: PlayType, last_card: Card) -> Vec<u32> {
		// Find and play any potential combos
		if play_type == PlayType::Clear {
			for i in (2..=4).rev() {
				let seq = find_biggest_card_sequence(&self.hand, i, last_card);

				if seq.len() != 0 {
					return seq;
				}
			}
		}

		// Finds the biggest
		match play_type {
			PlayType::Clear | PlayType::Single => {
				find_biggest_card_sequence(&self.hand, 1, last_card)
			}
			PlayType::Double => find_biggest_card_sequence(&self.hand, 2, last_card),
			PlayType::Triple => find_biggest_card_sequence(&self.hand, 3, last_card),
			PlayType::Quadruple => find_biggest_card_sequence(&self.hand, 4, last_card),
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

// Finds the first card sequence containing the biggest, same face cards.
fn find_biggest_card_sequence(hand: &Vec<Card>, length: usize, last_card: Card) -> Vec<u32> {
	match hand
		.iter()
		// Add index to iterator
		.enumerate()
		// Clone hand iter, so we can window it. (Shouldn't Enumerate<Self> support windowing?)
		.collect::<Vec<(usize, &Card)>>()
		// Divide into overlapping chunks of two
		.windows(length)
		// Reverse because we want to find
		.rev()
		// Find a sequence where all that cards are the same
		// and are the same or higher as the last_card.
		.find(|item| {
			item.iter().min().unwrap().1 == item.iter().max().unwrap().1 && item[0].1 >= &last_card
		}) {
		Some(cards) => {
			// Push indexes of cards to vec
			let mut vec = vec![];
			for (i, _) in cards {
				vec.push(*i as u32)
			}

			vec
		}
		// If no suitable sequence is found, return an empty Vec
		None => vec![],
	}
}