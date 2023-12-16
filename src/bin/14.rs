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
	let rows = input.lines().map(|line| Box::<[u8]>::from(line.as_bytes())).collect::<Box<_>>();
	let [x, _y] = [rows[0].len(), rows.len()];
	let mut grid = rows.into_iter().map(|row| row.into_iter().copied()).flatten().collect::<Box<_>>();

	let start = std::time::Instant::now();
	let mut last_status = std::time::Instant::now();
	let total = 1_000_000_000;
	for done in 0..total {
		if last_status.elapsed().as_secs() >= 1 {
			println!("Will be done within the next {} seconds", start.elapsed().as_secs() as u128*(total-done) as u128/done as u128);
			last_status = std::time::Instant::now();
		}
		// Up
		loop {
			let mut any = false;
			let mut top = grid.as_mut_ptr();
			let mut bottom = unsafe{grid.as_mut_ptr().add(x)};
			let end = unsafe{grid.as_ptr().add(grid.len())};
			while bottom as *const _< end {
				if unsafe{*bottom} == b'O' && unsafe{*top} == b'.' { unsafe{*top=b'O'; *bottom=b'.';} any=true; }
				bottom = unsafe{bottom.add(1)};
				top = unsafe{top.add(1)};
			}
			if !any { break; }
		}
		// Left
		loop {
			let mut any = false;
			let mut left = grid.as_mut_ptr();
			let mut right = unsafe{grid.as_mut_ptr().add(1)};
			let end = unsafe{grid.as_ptr().add(grid.len())};
			while right as *const _< end {
				let end = unsafe{grid.as_ptr().add(x)};
				while right as *const _< end {
					if unsafe{*right} == b'O' && unsafe{*left} == b'.' { unsafe{*left=b'O'; *right=b'.';} any=true; }
					left = right;
					right = unsafe{right.add(1)};
				}
				left = right;
				right = unsafe{right.add(1)};
			}
			if !any { break; }
		}
		// Down
		loop {
			let mut any = false;
			let mut top = grid.as_mut_ptr();
			let mut bottom = unsafe{grid.as_mut_ptr().add(x)};
			let end = unsafe{grid.as_ptr().add(grid.len())};
			while bottom as *const _< end {
				if unsafe{*top} == b'O' && unsafe{*bottom} == b'.' { unsafe{*top=b'.'; *bottom=b'O';} any=true; }
				bottom = unsafe{bottom.add(1)};
				top = unsafe{top.add(1)};
			}
			if !any { break; }
		}
		// Right
		loop {
			let mut any = false;
			let mut left = grid.as_mut_ptr();
			let mut right = unsafe{grid.as_mut_ptr().add(1)};
			let end = unsafe{grid.as_ptr().add(grid.len())};
			while right as *const _< end {
				let end = unsafe{grid.as_ptr().add(x)};
				while right as *const _< end {
					if unsafe{*left} == b'O' && unsafe{*right} == b'.' { unsafe{*left=b'.'; *right=b'O';} any=true; }
					left = right;
					right = unsafe{right.add(1)};
				}
				left = right;
				right = unsafe{right.add(1)};
			}
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
