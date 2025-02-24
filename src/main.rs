use infinite_debate::modules::entities;
use rand::prelude::IndexedRandom;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let philosophers = entities::get_philosopher_set()?;
    let player1_philosopher = philosophers.choose(&mut rand::rng()).unwrap();
    let player1 = entities::InPlayPhilosopher::new(player1_philosopher);
    println!("p1: {:?}\n", player1);
    let player2 = philosophers.choose(&mut rand::rng());
    println!("p2: {:?}\n", player2);
    let actions = entities::get_actions()?;
    for a in actions {
        println!("{:?}\n", a)
    }
    Ok(())
}
