#[macro_use]
extern crate strum_macros;

pub mod card;
use card::*;


pub trait Player {
	fn tick(&mut self, play_type: PlayType, last_card: Card) -> Vec<u32>;
	fn set_hand(&mut self, hand: Vec<Card>);
	fn get_mut_hand(&mut self) -> &mut Vec<Card>;
	fn get_hand(&self) -> &Vec<Card>;
	fn get_name(&self) -> &String;
}

pub struct Game {
	pub players: Vec<Box<Player>>,
	pub top_card: Option<Card>,
	pub play_type: PlayType,
	pub passes: usize,
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
			top_card: None,
			play_type: PlayType::Clear,
			passes: 0,
		}
	}

	// Primary game loop. This function runs an entire turn, for
	// all players
	pub fn tick(&mut self) -> Result<(), TickError> {
		use TickError::*;
		// Circumwent borrow of already mutable borrowed checking
		// rules.
		let number_of_players = self.players.len();

		for player in &mut self.players {
			loop {
				// Stores whether this player should play the next turn.
				// This happens, for example, when they clear the board
				// with a Face::Ten.
				let mut cleared_board = false;

				// If we have had a round of passes, reset the field
				if self.passes == number_of_players + 1 {
					self.passes = 0;
					self.top_card = None;
					self.play_type = PlayType::Double;
				}

				// Actual player tick.
				let play = player.tick(
					self.play_type,
					// Instead of playing with null values, if the bottom
					// card should be null, make it a three instead.
					self.top_card
						.unwrap_or(Card::new(Face::Three, Suit::Diamonds)),
				);

				// If it used to be a PlayType::Clear, we must know
				// assign one, based on the players play.
				if self.play_type == PlayType::Clear {
					self.play_type = match play.len() {
						// Same as a pass.
						0 => {
							self.passes += 1;
							continue;
						}
						1 => PlayType::Single,
						2 => PlayType::Double,
						3 => PlayType::Triple,
						//
						4 => PlayType::Quadruple,
						// Played too many cards.
						_ => return Err(IllegalNumberOfCards),
					}

				}

				// Compare PlayType to number of played cards.
				// If not a match, illegal move.
				if match self.play_type {
					PlayType::Single => 1,
					PlayType::Double => 2,
					PlayType::Triple => 3,
					PlayType::Clear => unreachable!(),
				} != play.len()
				{
					return Err(WrongNumberOfCards);
				}

				// Update for progress. Prints in the format:
				//     `{name} played {number of} '{card}'`
				// e.g `Big1 played 1 'Queen'`
				println!(
					"{} played {} '{}'",
					player.get_name(),
					play.len(),
					player.get_hand()[play[0] as usize].face
				);

				// Ten clears the board
				if player.get_hand()[play[0] as usize].face == Face::Ten {
					cleared_board = true;
				}

				// Remove played cards from players hand.
				let player_hand = player.get_mut_hand();
				play.iter().enumerate().for_each(|(e, index)| {
					player_hand.remove(*index as usize - e);
				});

				// If the player has no cards left in their hand, they
				// have won.
				if player_hand.len() == 0 {
					return Err(GameOver(player.get_name().to_string()));
				}

				// If the player didn't clear the board, the turn falls to the next guy.
				if cleared_board == false {
					break;
				}

				self.passes = 0;
				self.top_card = None;
				self.play_type = PlayType::Clear;
			}
		}
		println!();
		Ok(())
	}
}

pub enum TickError {
	// GameOver. (String) is the name of the winner.
	GameOver(String),
	// If `player` tries to play a card they don't have.
	NoneExistingCard,
	// If `player` tries to play, for example, three
	// cards on a two card turn.
	WrongNumberOfCards,
	// If `player` tries to play more than four cards.
	IllegalNumberOfCards,
	// If `player` tries to play the correct amount of
	// cards but not the same faced cards.
	NotIdenticalFaces,
}
