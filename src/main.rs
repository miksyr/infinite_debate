use infinite_debate::modules::entities;
use infinite_debate::modules::game_management;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (player1_hand, mut remaining_cards) = game_management::get_intial_deck().unwrap();
    println!("{:?}\n", player1_hand);
    println!("{:?}\n", remaining_cards);
    let next_cards = remaining_cards.draw_new_cards(2).unwrap();
    println!("next: {:?}\n", next_cards);
    let actions = entities::get_actions()?;
    for a in actions {
        println!("{:?}\n", a)
    }
    Ok(())
}
