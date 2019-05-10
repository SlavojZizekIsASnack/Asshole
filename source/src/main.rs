use std::collections::HashMap;

use game_lib::Game;

// fn main() {
// 	let mut g = Game::new(vec![
// 		rust::BigCombo::new("1"),
// 		rust::BigCombo::new("2"),
// 		rust::BigCombo::new("3"),
// 	]);

// 	loop {
// 		g.tick().unwrap();
// 	}

// }

fn main() {
	let runs = 10000000;

	let mut leaderboard = HashMap::new();

	for _ in 0..runs {
		let winner = play_game();
		let counter = leaderboard.entry(winner).or_insert(0);
		*counter += 1;
	}

	println!("{:#?}", leaderboard);
}


fn play_game() -> String {
	let mut g = Game::new(vec![
		rust::BigCombo::new("1"),
		rust::BigCombo::new("2"),
		rust::BigCombo::new("3"),
	]);

	loop {
		use game_lib::TickError::*;
		match g.tick() {
			Ok(_) => (),
			Err(GameOver(winner)) => return winner,
			_ => return String::from("Crashed"),
		};
	}
}