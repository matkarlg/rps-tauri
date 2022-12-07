#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]

mod game;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
fn main() {
	tauri::Builder::default()
		.invoke_handler(tauri::generate_handler![game::play, game::scoreboard])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
