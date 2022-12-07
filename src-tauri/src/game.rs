use rand::seq::IteratorRandom;
use std::str::FromStr;
use strum::{EnumIter, IntoEnumIterator};

#[tauri::command]
pub fn play(hand: &str) -> Result<String, String> {
	let player: Choice = hand.parse().map_err(|_| "Bad spelling. try again...")?;
	let computer = Choice::iter().choose(&mut rand::thread_rng()).unwrap();

	use Choice::*;
	let result = match (&player, &computer) {
		(Paper, Rock) | (Scissors, Paper) | (Rock, Scissors) => "You Win!",
		_ if player == computer => "Tie!",
		_ => "Computer Wins!",
	};

	Ok(result.into())
}

#[tauri::command]
pub fn scoreboard() -> String {
	todo!()
}

#[derive(Debug, EnumIter, PartialEq)]
enum Choice {
	Rock,
	Paper,
	Scissors,
}

impl FromStr for Choice {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.trim().to_lowercase().as_str() {
			"r" | "rock" => Ok(Choice::Rock),
			"p" | "paper" => Ok(Choice::Paper),
			"s" | "scissors" => Ok(Choice::Scissors),
			_ => Err(()),
		}
	}
}
