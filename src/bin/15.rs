advent_of_code::solution!(15);

#[fehler::throws(as Option)] pub fn part_one(input: &str) -> u32 {
	input.lines().map(|line| line.split(',')).flatten().map(|s| s.as_bytes().iter().fold(0u8, |hash, &c| (hash.wrapping_add(c)).wrapping_mul(17))).map(|u8| u8 as u32).sum()
}

#[fehler::throws(as Option)] pub fn part_two(input: &str) -> usize {
	let mut slots: [_; 256] = std::array::from_fn(|_| Vec::<(&str, usize)>::new());
	for step in input.trim().split(',') {
		let (ref label, focal) = step.split_once(['=','-']).unwrap();
		let slot = &mut slots[label.as_bytes().iter().fold(0u8, |hash, &c| (hash.wrapping_add(c)).wrapping_mul(17)) as usize];
		let operation = if step.contains('=') { '=' } else if step.ends_with('-') { '-' } else { unreachable!() };
		match operation {
			'=' => {
				let focal = focal.parse::<usize>().expect(&format!("'{focal}'"));
				if let Some(lens) = slot.iter_mut().find(|(lens,_)| lens == label) { lens.1 = focal } else { slot.push((label, focal)) }
				//println!("{:?}", slots.iter().enumerate().map(|(index, slot)| slot.iter().enumerate().map(move |(i, (_, focal))| ((1+index), (1+i), focal))).flatten().format(" "));
			},
			'-' => {
				if let Some(lens) = slot.iter().position(|(lens,_)| lens == label) { slot.remove(lens); }
				//println!("{:?}", slots.iter().enumerate().map(|(index, slot)| slot.iter().enumerate().map(move |(i, (_, focal))| ((1+index), (1+i), focal))).flatten().format(" "));
			}
			_ => unreachable!(),
		}
	}
	use itertools::Itertools;
	println!("{:?}", slots.iter().enumerate().map(|(index, slot)| slot.iter().enumerate().map(move |(i, (_, focal))| ((1+index), (1+i), focal))).flatten().format(" "));
	slots.iter().enumerate().map(|(index, slot)| slot.iter().enumerate().map(move |(i, (_, focal))| (1+index)*(1+i)*focal)).flatten().sum()
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
		assert_eq!(result, Some(145));
	}
}
