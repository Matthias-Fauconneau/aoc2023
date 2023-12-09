advent_of_code::solution!(6);

#[fehler::throws(as Option)]  pub fn part_one(input: &str) -> u32 {
	let (time, distance) = input.split_once('\n').unwrap();
	let [time, distance] = [time, distance].map(|s| s.split_once(':').unwrap().1.split_whitespace().map(|e| e.parse::<u32>().unwrap()));
	let races = time.zip(distance);
	races.map(|(time, distance)| {
		(1..time).filter(|hold| {
			hold*(time-hold) > distance
		}).count() as u32
	}).product()
}

#[fehler::throws(as Option)]  pub fn part_two(input: &str) -> u32 {
	let (time, distance) = input.split_once('\n').unwrap();
	let [time, distance] = [time, distance].map(|s| s.split_once(':').unwrap().1.chars().filter(|c| !c.is_whitespace()).collect::<String>().parse::<u64>().unwrap());
	(1..time).filter(|hold| {
		hold*(time-hold) > distance
	}).count() as u32
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_part_one() {
		let result = part_one(&advent_of_code::template::read_file("examples", DAY));
		assert_eq!(result, Some(288));
	}

	#[test]
	fn test_part_two() {
		let result = part_two(&advent_of_code::template::read_file("examples", DAY));
		assert_eq!(result, Some(71503));
	}
}
