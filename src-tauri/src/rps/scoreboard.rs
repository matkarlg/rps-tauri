use once_cell::sync::OnceCell;
use serde::Serialize;
use std::sync::Mutex;

use super::game::Outcome;

#[derive(Debug, Default, Serialize)]
pub struct Scoreboard {
	wins: u32,
	losses: u32,
	draws: u32,
}

impl Scoreboard {
	pub fn global() -> &'static Mutex<Self> {
		static INSTANCE: OnceCell<Mutex<Scoreboard>> = OnceCell::new();
		INSTANCE.get_or_init(|| Mutex::new(Scoreboard::default()))
	}

	pub fn add(score: &Outcome) -> &str {
		let mut global = Scoreboard::global().lock().unwrap();
		match score {
			Outcome::Win => {
				global.wins += 1;
				"You Win!"
			}
			Outcome::Lose => {
				global.losses += 1;
				"Computer Wins!"
			}
			Outcome::Draw => {
				global.draws += 1;
				"Tie!"
			}
		}
	}

	pub fn reset() {
		let mut global = Scoreboard::global().lock().unwrap();
		global.wins = 0;
		global.losses = 0;
		global.draws = 0;
	}

	pub fn show() -> serde_json::Value {
		let global = Scoreboard::global().lock().unwrap();
		serde_json::to_value(&*global).unwrap()
	}
}
