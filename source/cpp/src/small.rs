use game_lib::card::*;
use game_lib::Player;

use crate::{CVec, Play};

pub struct Small {
	hand: Vec<Card>,
	name: String,
}

impl Small {
	pub fn new(name: impl Into<String>) -> Box<Player> {
		Box::new(Self {
			hand: vec![],
			name: name.into(),
		})
	}
}

extern "C" {
	fn tick_unsafe(c_hand: CVec<Card>, play_type: PlayType, last_card: Card) -> Play;
}

impl Player for Small {
	fn tick(&mut self, play_type: PlayType, last_card: Card) -> Vec<u32> {
		unsafe { tick_unsafe(CVec::as_cvec(self.get_mut_hand()), play_type, last_card).into_vec() }
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
