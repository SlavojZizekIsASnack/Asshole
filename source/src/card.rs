use strum::IntoEnumIterator;

#[repr(C)]
#[derive(Clone, Copy, EnumIter, Display, PartialEq, Eq)]
pub enum Face {
	Ace,
	Two,
	Three,
	Four,
	Five,
	Six,
	Seven,
	Eight,
	Nine,
	Ten,
	Jack,
	Queen,
	King,
}

#[repr(C)]
#[derive(Clone, Copy, EnumIter, Display, PartialEq, Eq)]
pub enum Suit {
	Diamonds,
	Clubs,
	Hearts,
	Spades,
}

#[derive(Clone)]
pub struct Card {
	face: Face,
	suit: Suit,
}

impl Card {
	pub fn new(face: Face, suit: Suit) -> Card {
		Card { face, suit }
	}

	pub fn deck() -> Vec<Card> {
		let mut deck = Vec::with_capacity(52);
		for suit in Suit::iter() {
			for face in Face::iter() {
				deck.push(Card::new(face, suit))
			}
		}

		deck
	}
}

impl std::fmt::Display for Card {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{} of {}", self.face, self.suit)
	}
}
