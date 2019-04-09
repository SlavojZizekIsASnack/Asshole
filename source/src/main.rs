use game_lib::Game;

fn main() {
	let mut g = Game::new(vec![
		rust::Simple::new(),
		cpp::Simple::new(),
		cpp::Simple::new(),
	]);

	g.tick();
	g.tick();
}
