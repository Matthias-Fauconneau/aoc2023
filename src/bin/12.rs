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

#[fehler::throws(as Option)]  pub fn part_two(input: &str) -> usize {
	let lines = input.lines().collect::<Box<_>>();
	lines.iter().enumerate().map(|(i,line)| {
		let (springs, damaged) = line.split_once(' ').unwrap();
		let springs_utf = std::iter::repeat(springs).take(5).collect::<Box<_>>().join("?");
		let damaged = std::iter::repeat(damaged).take(5).collect::<Box<_>>().join(",");
		let springs = springs_utf.as_bytes();
		let damaged = damaged.split(',').map(|n| n.parse::<usize>().unwrap()).collect::<Box<_>>();
		let total_damaged = damaged.iter().sum::<usize>();
		if total_damaged < springs.len() {
			//let max_operational_seq_len = springs.len()-total_damaged;
			let unknown = springs.into_iter().filter(|&&c| c == b'?').count();
			println!("{unknown} {}", 1<<unknown);
			//println!("{max_operational_seq_len} {} {}", damaged.len(), max_operational_seq_len.pow(damaged.len() as u32));
			let count = /*if 1<<unknown < max_operational_seq_len.pow(damaged.len() as u32)*/ {
				(0..(1<<unknown)).filter(|candidate| {
					let mut springs = springs.into_iter().map({let mut unknown_index = 0; move |&c| match c {
						b'.' => b'.',
						b'#' => b'#',
						b'?' => { let c = if candidate & (1<<unknown_index) == 0 { b'.' } else { b'#' }; unknown_index += 1; c }
						_ => unreachable!()
					}});
					for (i, &group) in damaged.iter().enumerate() {
						while let Some(b'.') = springs.next() {}
						for _ in 1..group { if let Some(b'.') = springs.next() { return false; } }
						if i < damaged.len()-1  { if let Some(b'#') = springs.next() { return false; } }
					}
					true
				}).count()
			} ;/*else {
				let mut count = 0;				
				'next_candidate: for mut candidate in 0..max_operational_seq_len.pow(damaged.len() as u32) {
					let mut i = 0;
					for damaged in &*damaged {
			 			let operational_until = i+candidate%max_operational_seq_len;
			 			candidate /= max_operational_seq_len;
			 			if operational_until+damaged > springs.len() { continue; }
			 			while i < operational_until {
			 				if springs[i] == b'#' { continue 'next_candidate; }
			 				i += 1;
			 			}
			 			let damaged_until = i+damaged;
						while i < damaged_until {
			 				if springs[i] != b'#' { continue 'next_candidate; }
			 				i += 1;
			 			}
					}
					count += 1;
				}
				count
			};*/
			println!("{i}/{} {springs_utf} {damaged:?} {count}", lines.len());
			count
		} else {
			println!("{springs_utf} {damaged:?}");
			0
		}
	}).sum()
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
		assert_eq!(result, Some(525152));
	}
}
