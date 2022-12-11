use rand::seq::IteratorRandom;
use serde::Serialize;
use std::str::FromStr;
use strum::{EnumIter, IntoEnumIterator};

use super::scoreboard::Scoreboard;

pub fn game(hand: &str) -> Result<serde_json::Value, String> {
	let player: Choice = hand.parse().map_err(|_| "Bad spelling. try again...")?;
	let computer = Choice::iter().choose(&mut rand::thread_rng()).unwrap();

	let result = play(&player, &computer);

	Scoreboard::add(&result);

	Ok(serde_json::to_value((result, computer)).unwrap())
}

pub fn play(player: &Choice, computer: &Choice) -> Outcome {
	use Choice::*;
	match (&player, &computer) {
		(Paper, Rock) | (Scissors, Paper) | (Rock, Scissors) => Outcome::Win,
		_ if player == computer => Outcome::Draw,
		_ => Outcome::Lose,
	}
}

#[derive(Debug, Serialize)]
pub enum Outcome {
	Win = 6,
	Lose = 0,
	Draw = 3,
}

#[derive(Debug, EnumIter, PartialEq, Serialize, Clone, Copy)]
pub enum Choice {
	Rock = 1,
	Paper,
	Scissors,
}

impl FromStr for Choice {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.trim().to_lowercase().as_str() {
			"a" | "x" | "rock" => Ok(Choice::Rock),
			"b" | "y" | "paper" => Ok(Choice::Paper),
			"c" | "z" | "scissors" => Ok(Choice::Scissors),
			_ => Err(()),
		}
	}
}
