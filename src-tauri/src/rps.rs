mod aoc;
mod game;
mod scoreboard;

#[tauri::command]
pub fn aoc(guide: &str) -> Result<u32, String> {
	aoc::aoc(guide)
}

#[tauri::command]
pub fn show_scoreboard() -> serde_json::Value {
	scoreboard::Scoreboard::show()
}

#[tauri::command]
pub fn play_game(hand: &str) -> Result<serde_json::Value, String> {
	game::game(hand)
}

#[tauri::command]
pub fn reset_scoreboard() {
	scoreboard::Scoreboard::reset()
}
