use game_lib::card::*;
use game_lib::Player;

pub struct Simple {
	hand: Vec<Card>,
}

impl Simple {
	pub fn new() -> Box<Player> {
		Box::new(Self { hand: vec![] })
	}
}

impl Player for Simple {
	fn tick(&mut self, play_type: PlayType, last_card: Card) -> Card {
		self.hand[0]
	}

	fn set_hand(&mut self, hand: Vec<Card>) {
		self.hand = hand;
	}

	fn mut_hand(&mut self) -> &mut Vec<Card> {
		&mut self.hand
	}

	fn get_hand(&self) -> &Vec<Card> {
		&self.hand
	}
}
