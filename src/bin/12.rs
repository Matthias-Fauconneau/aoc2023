advent_of_code::solution!(12);

#[fehler::throws(as Option)] pub fn part_one(input: &str) -> usize {
	input.lines().map(|line| {
		let (springs, damaged) = line.split_once(' ').unwrap();
		let damaged = damaged.split(',').map(|n| n.parse::<usize>().unwrap()).collect::<Box<_>>();
		let unknown = springs.chars().filter(|&c| c == '?').count();
		println!("{springs} {damaged:?} {unknown}");
		(0..(1<<unknown)).map(|candidate| {
			let mut unknown_index = 0;
			springs.chars().map(move |c| match c {
				'.' => ' ',
				'#' => '#',
				'?' => { let c = if candidate & (1<<unknown_index) == 0 { ' ' } else { '#' }; unknown_index += 1; c }
				_ => unreachable!()
			}).collect::<String>()
		}).filter(|row| {
			let row = row.split_whitespace().map(|group| group.len()).collect::<Box<_>>();
			&*row == &*damaged
		}).count()
	}).sum()
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
		assert_eq!(result, Some(21));
	}

	#[test]
	fn test_part_two() {
		let result = part_two(&advent_of_code::template::read_file("examples", DAY));
		assert_eq!(result, None);
	}
}
