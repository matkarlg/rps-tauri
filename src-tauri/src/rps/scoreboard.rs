use serde::Serialize;
use std::sync::Mutex;

use super::game::Outcome;

#[derive(Debug, Serialize)]
pub struct Scoreboard {
	wins: u32,
	losses: u32,
	draws: u32,
}

impl Scoreboard {
	pub fn global() -> &'static Mutex<Self> {
		static SCOREBOARD: Mutex<Scoreboard> = Mutex::new(Scoreboard::new());
		&SCOREBOARD
	}

	pub fn add(score: &Outcome) -> &str {
		let _global = Scoreboard::global();
		let mut global = _global.lock().unwrap();
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
		*Scoreboard::global().lock().unwrap() = Self::new()
	}

	pub fn show() -> serde_json::Value {
		serde_json::to_value(&*Scoreboard::global().lock().unwrap()).unwrap()
	}

	const fn new() -> Self {
		Self {
			wins: 0,
			losses: 0,
			draws: 0,
		}
	}
}
