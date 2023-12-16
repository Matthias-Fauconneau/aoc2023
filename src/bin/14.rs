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

#[fehler::throws(as Option)]  pub fn part_two(input: &str) -> usize {
	let mut rows = input.lines().map(|line| Box::<[u8]>::from(line.as_bytes())).collect::<Box<_>>();
	let start = std::time::Instant::now();
	let mut last_status = std::time::Instant::now();
	let total = 1_000_000_000;
	for done in 0..total {
		if last_status.elapsed().as_secs() >= 1 {
			println!("Will be done within the next {} seconds", start.elapsed().as_secs() as u128*(total-done) as u128/done as u128);
			last_status = std::time::Instant::now();
		}
		// North
		loop {
			let mut any = false;
			(0..rows.len()).map_windows(|&[top, bottom]|  {
				for column in 0..rows[0].len() {
					let [top, bottom] = rows.get_many_mut([top,bottom]).unwrap().map(|row| &mut row[column]);
					if *top == b'.' && *bottom == b'O'  { std::mem::swap(top, bottom); any=true; }
				}
			}).for_each(|_|{});
			if !any { break; }
		}
		// West
		for _ in 0..rows[0].len() {
			let mut any = false;
			(0..rows[0].len()).map_windows(|&[left, right]|  {
				for row in &mut *rows {
					let [left, right] = row.get_many_mut([left,right]).unwrap();
					if *left == b'.' && *right == b'O'  { std::mem::swap(left, right); any=true; }
				}
			}).for_each(|_|{});
			if !any { break; }
		}
		// South
		for _ in 0..rows.len() {
			let mut any = false;
			(0..rows.len()).map_windows(|&[top, bottom]|  {
				for column in 0..rows[0].len() {
					let [top, bottom] = rows.get_many_mut([top,bottom]).unwrap().map(|row| &mut row[column]);
					if *bottom == b'.' && *top == b'O'  { std::mem::swap(top, bottom); any=true; }
				}
			}).for_each(|_|{});
			if !any { break; }
		}
		// East
		for _ in 0..rows[0].len() {
			let mut any = false;
			(0..rows[0].len()).map_windows(|&[left, right]|  {
				for row in &mut *rows {
					let [left, right] = row.get_many_mut([left,right]).unwrap();
					if *right == b'.' && *left == b'O'  { std::mem::swap(left, right); any=true; }
				}
			}).for_each(|_|{});
			if !any { break; }
		}
	}
	rows.into_iter().enumerate().map(|(i,row)| row.into_iter().filter(|&&c| c==b'O').count() * (rows.len()-i)).sum()
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
		assert_eq!(result, Some(64));
	}
}
