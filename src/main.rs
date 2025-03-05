mod entities;
mod game_management;
mod player;
mod rendering;
mod test_utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let game_app = rendering::GameApp::new();
    let terminal = ratatui::init();
    let app_result = game_app.run(terminal);
    ratatui::restore();
    Ok(app_result?)
}
