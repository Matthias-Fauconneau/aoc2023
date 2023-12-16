advent_of_code::solution!(15);

#[fehler::throws(as Option)] pub fn part_one(input: &str) -> u32 {
	input.lines().map(|line| line.split(',')).flatten().map(|s| s.as_bytes().iter().fold(0u8, |hash, c| (hash+c)*17)).map(|u8| u8 as u32).sum()
}

#[fehler::throws(as Option)]  pub fn part_two(_input: &str) -> u32 {
	fehler::throw!();
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_part_one() {
		let result = part_one(&advent_of_code::template::read_file("examples", DAY));
		assert_eq!(result, Some(1320));
	}

	#[test]
	fn test_part_two() {
		let result = part_two(&advent_of_code::template::read_file("examples", DAY));
		assert_eq!(result, None);
	}
}
