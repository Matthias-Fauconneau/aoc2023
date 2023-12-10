#![allow(non_upper_case_globals)]
advent_of_code::solution!(7);

#[fehler::throws(as Option)] pub fn part_one(input: &str) -> u32 {
	const cards : [char; 13] = ['2','3','4','5','6','7','8','9','T','J','Q','K','A'];
	let mut hands = input.lines().map(|line| {
		let (hand, bid) = line.split_once(' ').unwrap();
		fn from_iter<T, const N: usize>(mut iter: impl Iterator<Item=T>) -> [T; N] { [(); N].map(|_| iter.next().unwrap()) }
		let hand : [_; 5] = from_iter(hand.chars().map(|c| cards.iter().position(|&v| c==v).unwrap()));
		let mut histogram = [0; 13];
		for card in hand { histogram[card] += 1; }
		histogram.sort();
		histogram.reverse();
		let type_rank = match histogram {
			[5, ..] => 7,
			[4, ..] => 6,
			[3, 2, ..] => 5,
			[3, ..] => 4,
			[2, 2, ..] => 3,
			[2, ..] => 2,
			_ => 1,
		};
		(type_rank, hand, bid.parse::<u32>().unwrap())
	}).collect::<Box<_>>();
	hands.sort();
	hands.iter().enumerate().map(|(index, (_, _, bid))| {
		let rank = index+1;
		println!("{rank} {bid}");
		rank as u32*bid
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
		assert_eq!(result, Some(6440));
	}

	#[test]
	fn test_part_two() {
		let result = part_two(&advent_of_code::template::read_file("examples", DAY));
		assert_eq!(result, None);
	}
}
