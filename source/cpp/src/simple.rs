use game_lib::card::*;
use game_lib::Player;

extern "C" {
	fn tick_unsafe(
		hand_ptr: *mut Card,
		hand_len: libc::size_t,
		play_type: PlayType,
		last_card: Card,
	) -> Card;
}

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
		unsafe {
			tick_unsafe(
				self.hand.as_mut_ptr(),
				self.hand.len(),
				play_type,
				last_card,
			)
		}
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
