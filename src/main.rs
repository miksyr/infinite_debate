//use infinite_debate::modules::game_management;
use infinite_debate::modules::rendering;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let game_app = rendering::GameApp::new();
    let terminal = ratatui::init();
    let app_result = game_app.run(terminal);
    ratatui::restore();
    //let game_board = game_management::GameBoard::new();
    //println!("{:?}\n\n", game_board);

    //let (player1_hand, mut remaining_cards) = game_management::get_intial_deck().unwrap();
    //println!("{:?}\n", player1_hand);
    //println!("{:?}\n", remaining_cards);
    //println!("remaining: {:?}\n", remaining_cards.num_remaining_cards());
    //let next_cards = remaining_cards.draw_new_cards(2).unwrap();
    //println!("next: {:?}\n", next_cards);
    //println!("remaining: {:?}", remaining_cards.num_remaining_cards());
    Ok(app_result?)
}
