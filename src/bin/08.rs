advent_of_code::solution!(8);

#[fehler::throws(as Option)] pub fn part_one(input: &str) -> usize {
	let (sequence, map) = input.split_once("\n\n").unwrap();
	let nodes = map.lines().map(|line| {
		let (key, next) = line.split_once('=').unwrap();
		let key = key.trim();
		let (left, right) = next.trim().strip_prefix('(').unwrap().strip_suffix(')').unwrap().split_once(", ").unwrap();
		(key, [left, right])
	}).collect::<Box<_>>();
	let start = nodes.iter().position(|&(k,_)| k=="AAA").unwrap();
	let end = nodes.iter().position(|&(k,_)| k=="ZZZ").unwrap();
	let nodes = nodes.iter().map(|(_, nexts)| nexts.map(|next| nodes.iter().position(|&(k,_)| k==next).unwrap())).collect::<Box<_>>();
	sequence.chars().cycle().scan(start, |position, step| {
		*position = nodes[*position][match step { 'L' => 0, 'R' => 1, _ => unreachable!()}];
		Some(*position)
	}).position(|position| position == end).unwrap()+1
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
		assert_eq!(result, Some(6));
	}

	#[test]
	fn test_part_two() {
		let result = part_two(&advent_of_code::template::read_file("examples", DAY));
		assert_eq!(result, None);
	}
}
