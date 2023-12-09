advent_of_code::solution!(4);

#[fehler::throws(as Option)] pub fn part_one(input: &str) -> u32 {
	input.lines().filter_map(|line| {
		let (_, card) = line.split_once(':').unwrap();
		let (win, has) = card.split_once('|').unwrap();
		let [win, has] = [win, has].map(|list| list.trim().split_whitespace().map(|e| e.parse::<u32>()));
		let win = win.collect::<Box<_>>();
		let count = has.filter(|has| win.contains(has)).count();
		if count == 0 { None } else { Some(dbg!(2u32.pow(count as u32 -1))) }
	}).sum()
}

#[fehler::throws(as Option)]  pub fn part_two(input: &str) -> u32 {
	let cards = input.lines().map(|line| {
		let (_, card) = line.split_once(':').unwrap();
		let (win, has) = card.split_once('|').unwrap();
		let [win, has] = [win, has].map(|list| list.trim().split_whitespace().map(|e| e.parse::<u32>()));
		let win = win.collect::<Box<_>>();
		has.filter(|has| win.contains(has)).count()
	}).collect::<Box<_>>();
	fn f(cards: &[usize], start: usize, end: usize) -> u32 {
		cards[start..end].iter().enumerate().map(move |(i,&card)| {
			//println!("{} => {} {}", start+i, start + i + 1, start+i+1+card);
			let start = start + i + 1;
			1+f(cards, start, start+card)
		}).sum()
	}
	f(&cards, 0, cards.len())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_part_one() {
		let result = part_one(&advent_of_code::template::read_file("examples", DAY));
		assert_eq!(result, Some(13));
	}

	#[test]
	fn test_part_two() {
		let result = part_two(&advent_of_code::template::read_file("examples", DAY));
		assert_eq!(result, Some(30));
	}
}
