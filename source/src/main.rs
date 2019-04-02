use game_lib::Game;

fn main() {
	let mut g = Game::new(vec![
		cpp::simple::play,
		rust::simple::play,
		cpp::simple::play,
	]);

	g.tick();
}
