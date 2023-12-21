advent_of_code::solution!(21);

#[fehler::throws(as Option)] pub fn part_one(input: &str) -> usize {
	let rows = input.lines().map(|line| line.chars().collect::<Box<_>>()).collect::<Box<_>>();
	fn step(rows: &[Box<[char]>], [i,j]: [i32; 2], steps: usize) -> Vec<[i32; 2]> {
		if steps == 0 { return vec![[i,j]]; }
		[[-1,0],[0,-1],[0,1],[1,0]].iter().map(|[di, dj]| {
			let [i,j] = [i+di, j+dj];
			if i<0 || i >=rows.len() as _ || j<0 || j >= rows[0].len() as _ || rows[i as usize][j as usize] == '#' { return Vec::new(); }
			step(rows, [i,j], steps-1)
		}).reduce(|mut a,b| { for b in b { if !a.contains(&b) { a.push(b) }} a }).unwrap()
	}
	step(&*rows, rows.iter().enumerate().find_map(|(i,row)| row.iter().enumerate().find_map(|(j,&c)| (c=='S').then_some(j)).map(|j| [i,j])).unwrap().map(|u| u as _), 64).len()
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
