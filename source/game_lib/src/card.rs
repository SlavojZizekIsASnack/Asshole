use std::cmp::Ordering;
use strum::IntoEnumIterator;

#[repr(C)]
#[derive(Debug, Clone, Copy, EnumIter, Display, PartialEq, Eq)]
pub enum PlayType {
	Clear,
	Single,
	Double,
	Triple,
	Quadruple,
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
	Jack,
	Queen,
	King,
	Ace,
	Two,
	Ten,
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
#[derive(Debug, Clone, Copy)]
pub struct Card {
	pub face: Face,
	pub suit: Suit,
}

impl Card {
	pub fn new(face: Face, suit: Suit) -> Card {
		Card { face, suit }
	}

	// Return a Vec<Card> of size 52
	pub fn deck() -> Vec<Card> {
		let mut deck = Vec::with_capacity(52);
		for suit in Suit::iter() {
			for face in Face::iter() {
				deck.push(Card::new(face, suit))
			}
		}

		deck
	}

	pub fn shuffled_deck() -> Vec<Card> {
		use rand::seq::SliceRandom;
		use rand::thread_rng;

		let mut deck = Self::deck();
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

// Suit doesn't matter for ordering. They're always the same
impl PartialOrd for Card {
	fn partial_cmp(&self, other: &Card) -> Option<Ordering> {
		self.face.partial_cmp(&other.face)
	}
}

// Suit doesn't matter for ordering. They're always the same
impl Ord for Card {
	fn cmp(&self, other: &Card) -> Ordering {
		self.face.cmp(&other.face)
	}
}

// Suit doesn't matter for equality testing. They're always the same
impl PartialEq for Card {
	fn eq(&self, other: &Card) -> bool {
		self.face.eq(&other.face)
	}
}

// Suit doesn't matter for equality testing. They're always the same
impl Eq for Card {}