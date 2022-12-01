use itertools::Itertools;

const INPUT: &str = include_str!("real_input.txt");

fn calories(input: &str) -> impl Iterator<Item = u64> + '_ {
	input.split("\n\n").map(|s| {
		s.lines()
			.map(|l| l.parse::<u64>().expect("invalid input"))
			.sum()
	})
}

fn part1(calories: impl Iterator<Item = u64>) -> u64 {
	calories.max().unwrap_or_default()
}

fn part2(calories: impl Iterator<Item = u64>) -> u64 {
	calories.sorted_unstable_by(|a, b| b.cmp(a)).take(3).sum()
}

fn main() {
	println!("--- day1 ---");
	println!("part 1 => {}", part1(calories(INPUT)));
	println!("part 2 => {}", part2(calories(INPUT)));
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = include_str!("test_input.txt");

	#[test]
	fn test_part1() {
		assert_eq!(part1(calories(INPUT)), 24000);
	}
	
	#[test]
	fn test_part2() {
		assert_eq!(part2(calories(INPUT)), 45000);
	}
}