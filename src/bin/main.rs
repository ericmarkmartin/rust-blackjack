use blackjack::Game;

fn main() -> std::io::Result<()> {
    Game::new().play()?;
    println!("Thanks for playing!");
    Ok(())
}
