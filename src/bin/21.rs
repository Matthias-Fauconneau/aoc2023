advent_of_code::solution!(21);

#[fehler::throws(as Option)] pub fn part_one(input: &str) -> usize {
	let rows = input.lines().map(|line| line.chars().collect::<Box<_>>()).collect::<Box<_>>();
	let mut steps = vec![rows.iter().enumerate().find_map(|(i,row)| row.iter().enumerate().find_map(|(j,&c)| (c=='S').then_some(j)).map(|j| [i as _,j as _])).unwrap()];
	for _ in 0..64 {
		let mut next : Vec<[u32; 2]>= vec![];
		for [i,j] in steps {
			for [di,dj] in [[-1,0],[0,-1],[0,1],[1,0]] {
				let [i,j] = [(i as i32+di) as u32, (j as i32+dj) as u32];
				if !(i>=rows.len() as _ || j>=rows[0].len() as _ || rows[i as usize][j as usize] == '#') && !next.contains(&[i,j]) { next.push([i,j]); }
			}
		}
		steps = next;
	}
	steps.len()
}

#[fehler::throws(as Option)] pub fn part_two(input: &str) -> usize {
	let rows = input.lines().map(|line| line.chars().collect::<Box<_>>()).collect::<Box<_>>();
	let mut steps = vec![rows.iter().enumerate().find_map(|(i,row)| row.iter().enumerate().find_map(|(j,&c)| (c=='S').then_some(j)).map(|j| [i as _,j as _])).unwrap()];
	#[allow(non_snake_case)] let [I, J] = [rows.len(), rows[0].len()];
	for _ in 0..26501365/*(2^2)×(5^2)×7×(17^2)×131+65*/ {
		let mut next : Vec<[usize; 2]>= vec![];
		for [i,j] in steps {
			for [di,dj] in [[-1,0],[0,-1],[0,1],[1,0]] {
				let [i,j] = [(i as isize+di) as usize, (j as isize+dj) as usize];
				if !(rows[((I<<32) + i)%I][((J<<32) +j)%J] == '#') && !next.contains(&[i,j]) { next.push([i,j]); }
			}
		}
		steps = next;
	}
	steps.len()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_part_one() {
		let result = part_one(&advent_of_code::template::read_file("examples", DAY));
		assert_eq!(result, Some(42));
	}

	#[test]
	fn test_part_two() {
		let result = part_two(&advent_of_code::template::read_file("examples", DAY));
		assert_eq!(result, Some(16733044));
	}
}
