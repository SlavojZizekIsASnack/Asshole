use game_lib::Game;

fn main() {
	let mut g = Game::new(vec![
		rust::Big::new("Big1"),
		rust::Big::new("Big2"),
		rust::Big::new("Big3"),
	]);

	loop {
		use game_lib::TickError::*;
		match g.tick() {
			Err(GameOver(winner)) => {
				println!("\n{} won!", winner);
				break;
			}
			_ => continue,
		};
	}
}