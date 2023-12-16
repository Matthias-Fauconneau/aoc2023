#![feature(iter_map_windows, get_many_mut)]
advent_of_code::solution!(14);

#[fehler::throws(as Option)] pub fn part_one(input: &str) -> usize {
	let mut rows = input.lines().map(|line| Box::<[u8]>::from(line.as_bytes())).collect::<Box<_>>();
	for _ in 0..rows.len() {
		(0..rows.len()).map_windows(|&[top, bottom]|  {
			for column in 0..rows[0].len() {
				let [top, bottom] = rows.get_many_mut([top,bottom]).unwrap().map(|row| &mut row[column]);
				if *top == b'.' && *bottom == b'O'  { std::mem::swap(top, bottom) }
			}
		}).for_each(|_|{});
	}
	rows.into_iter().enumerate().map(|(i,row)| row.into_iter().filter(|&&c| c==b'O').count() * (rows.len()-i)).sum()
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
		assert_eq!(result, Some(136));
	}

	#[test]
	fn test_part_two() {
		let result = part_two(&advent_of_code::template::read_file("examples", DAY));
		assert_eq!(result, None);
	}
}
