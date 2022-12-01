#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]

use rand::{
	distributions::{Distribution, Standard},
	Rng,
};
use std::str::FromStr;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn game(hand: &str) -> Result<String, String> {
	let player: Choice = hand.parse()?;
	let computer: Choice = rand::random();

	use self::Choice::*;
	let result = match (&player, &computer) {
		(Paper, Rock) | (Scissors, Paper) | (Rock, Scissors) => "You Win!",
		_ if player == computer => "Tie!",
		_ => "Computer Wins!",
	};

	Ok(result.into())
}

fn main() {
	tauri::Builder::default()
		.invoke_handler(tauri::generate_handler![game])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}

#[derive(Debug, PartialEq)]
enum Choice {
	Rock,
	Paper,
	Scissors,
}

impl FromStr for Choice {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.trim().to_lowercase().as_str() {
			"r" | "rock" => Ok(Choice::Rock),
			"p" | "paper" => Ok(Choice::Paper),
			"s" | "scissors" => Ok(Choice::Scissors),
			_ => Err("Bad spelling. try again...".into()),
		}
	}
}

impl Distribution<Choice> for Standard {
	fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Choice {
		match rng.gen_range(0..3) {
			0 => Choice::Rock,
			1 => Choice::Paper,
			_ => Choice::Scissors,
		}
	}
}
