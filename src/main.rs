use infinite_debate::modules::entities;
use rand::prelude::IndexedRandom;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let philosophers = entities::get_philosopher_set()?;
    let player1_philosopher = philosophers.choose(&mut rand::rng()).unwrap();
    let player1 = entities::InPlayPhilosopher {
        philosopher: player1_philosopher,
        current_damage: 0,
        modifiers: None,
    };
    let player1 = philosophers.choose(&mut rand::rng());
    let player2 = philosophers.choose(&mut rand::rng());
    println!("p1: {:?}\n", player1);
    println!("p2: {:?}\n", player2);
    Ok(())
}
