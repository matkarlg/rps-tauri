#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]

mod rps;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
fn main() {
	tauri::Builder::default()
		.invoke_handler(tauri::generate_handler![
			rps::aoc,
			rps::play_game,
			rps::show_scoreboard,
			rps::reset_scoreboard,
		])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
