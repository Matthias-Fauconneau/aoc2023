advent_of_code::solution!(21);

#[fehler::throws(as Option)] pub fn part_one(input: &str) -> usize {
	let rows = input.lines().map(|line| line.chars().collect::<Box<_>>()).collect::<Box<_>>();
	let mut steps = vec![(rows.iter().enumerate().find_map(|(i,row)| row.iter().enumerate().find_map(|(j,&c)| (c=='S').then_some(j)).map(|j| [i as _,j as _])).unwrap(), 1)];
	for _ in 0..64 {
		let mut next : Vec<([u32; 2], usize)>= vec![];
		for ([i,j], count) in steps {
			for [di,dj] in [[-1,0],[0,-1],[0,1],[1,0]] {
				let [i,j] = [(i as i32+di) as u32, (j as i32+dj) as u32];
				if !(i>=rows.len() as _ || j>=rows[0].len() as _ || rows[i as usize][j as usize] == '#') {
					if let Some((_, next_count)) = next.iter_mut().find(|(position,_)| position==&[i,j]) { *next_count += count }
					else { next.push(([i,j], count)); }
				}
			}
		}
		steps = next;
	}
	steps.iter().map(|(_,_count)| 1).sum()
}

#[fehler::throws(as Option)] pub fn part_two(_input: &str) -> u32 {
	fehler::throw!();
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_part_one() {
		let result = part_one(&advent_of_code::template::read_file("examples", DAY));
		assert_eq!(result, Some(16));
	}

	#[test]
	fn test_part_two() {
		let result = part_two(&advent_of_code::template::read_file("examples", DAY));
		assert_eq!(result, None);
	}
}
