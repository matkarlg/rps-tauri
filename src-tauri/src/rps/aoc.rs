use super::game::{play, Choice};

pub fn aoc(guide: &str) -> Result<u32, String> {
	let results: u32 = guide
		.lines()
		.map(|line| {
			let choices: Vec<Result<Choice, ()>> = line
				.split_whitespace()
				.map(|c| c.parse::<Choice>())
				.collect();

			let computer = choices
				.get(0)
				.ok_or("Failed parsing")?
				.map_err(|_| "Failed parsing")?;

			let player = choices
				.get(1)
				.ok_or("Failed parsing")?
				.map_err(|_| "Failed parsing")?;

			let result = play(&player, &computer);

			Ok(result as u32 + player as u32)
		})
		.sum::<Result<u32, String>>()?;

	Ok(results)
}
