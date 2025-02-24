use infinite_debate::modules::entities;
use infinite_debate::modules::game_management;
use rand::prelude::IndexedRandom;
use rand::Rng;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let player1_hand = game_management::get_intial_deck();
    println!("{:?}\n", player1_hand);
    let actions = entities::get_actions()?;
    for a in actions {
        println!("{:?}\n", a)
    }
    Ok(())
}
