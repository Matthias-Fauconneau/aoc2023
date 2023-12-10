#![feature(array_windows)]
advent_of_code::solution!(9);

#[fehler::throws(as Option)] pub fn part_one(input: &str) -> i32 {
	input.lines().map(|line| {
		let mut stack = Vec::from([line.split_whitespace().map(|s| s.parse::<i32>().expect(s)).collect::<Box<_>>()]);
		loop {
			let next = stack.last().unwrap().array_windows().map(|[prev, next]| next-prev).collect::<Box<_>>();
			if next.iter().all(|&r| r==0) { break; }
			stack.push(next);
		}
		stack.iter().map(|history| history.last().unwrap()).sum::<i32>()
	}).sum()
}

#[fehler::throws(as Option)]  pub fn part_two(input: &str) -> i32 {
	/*input.lines().map(|line| {
		let mut stack = Vec::from([line.split_whitespace().map(|s| s.parse::<i32>().expect(s)).collect::<Box<_>>()]);
		loop {
			let next = stack.last().unwrap().array_windows().map(|[prev, next]| next-prev).collect::<Box<_>>();
			if next.iter().all(|&r| r==0) { break; }
			stack.push(next);
		}
		dbg!(-stack.iter().fold(0, |first, history| history.first().unwrap()-first))
	}).sum()*/
	input.lines().map(|line| {
		let mut stack = Vec::from([line.split_whitespace().map(|s| s.parse::<i32>().expect(s)).collect::<Box<_>>()]);
		stack[0].reverse();
		loop {
			let next = stack.last().unwrap().array_windows().map(|[prev, next]| next-prev).collect::<Box<_>>();
			if next.iter().all(|&r| r==0) { break; }
			stack.push(next);
		}
		stack.iter().map(|history| history.last().unwrap()).sum::<i32>()
	}).sum()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_part_one() {
		let result = part_one(&advent_of_code::template::read_file("examples", DAY));
		assert_eq!(result, Some(114));
	}

	#[test]
	fn test_part_two() {
		let result = part_two(&advent_of_code::template::read_file("examples", DAY));
		assert_eq!(result, Some(2));
	}
}
