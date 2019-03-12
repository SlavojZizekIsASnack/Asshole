use strum::IntoEnumIterator;

#[repr(C)]
#[derive(Debug, Clone, Copy, EnumIter, Display, PartialEq, Eq, PartialOrd, Ord)]
pub enum PlayType {
	Single,
	Double,
	Triple,
	Clear,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, EnumIter, Display, PartialEq, Eq, PartialOrd, Ord)]
pub enum Face {
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
	Ace,
	Two,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, EnumIter, Display, PartialEq, Eq, PartialOrd, Ord)]
pub enum Suit {
	Diamonds,
	Clubs,
	Hearts,
	Spades,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Card {
	face: Face,
	suit: Suit,
}

impl Card {
	pub fn new(face: Face, suit: Suit) -> Card {
		Card { face, suit }
	}

	// Return a Vec<Card> of size 52
	pub fn deck() -> Vec<Card> {
		use rand::seq::SliceRandom;
		use rand::thread_rng;

		let mut deck = Vec::with_capacity(52);
		for suit in Suit::iter() {
			for face in Face::iter() {
				deck.push(Card::new(face, suit))
			}
		}
		let mut rng = thread_rng();
		deck.shuffle(&mut rng);

		deck
	}
}

impl std::fmt::Display for Card {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{} of {}", self.face, self.suit)
	}
}
