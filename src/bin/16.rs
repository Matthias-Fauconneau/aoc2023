advent_of_code::solution!(16);

#[fehler::throws(as Option)] pub fn part_one(input: &str) -> usize {
	let ref mut geometry = input.lines().map(|line| line.chars().collect::<Box<_>>()).collect::<Box<_>>();
	let ref mut energy = geometry.iter().map(|line| line.iter().map(|_| 0).collect::<Box<_>>()).collect::<Box<_>>();
	//enum Direction { Up, Left, Right, Down }
	//struct Beam { position: [u32; 2], direction: Direction }
	fn beam(geometry: &[Box<[char]>], [mut i, mut j]: [isize; 2], [mut di, mut dj]: [isize; 2], ref mut energy: &mut [Box<[u32]>]) {
		loop {
			if i < 0  || i >= geometry.len() as isize || j<0 || j >= geometry[0].len() as isize || energy[i as usize][j as usize] >= 4 { return; }
			energy[i as usize][j as usize] += 1;
			match geometry[i as usize][j as usize] {
				'.' => {},
				'/' => [di, dj] = match [di, dj] { [di, 0] => [0, -di], [0, dj] => [-dj, 0], _ => unreachable!() },
				'\\' => [di, dj] = match [di, dj] { [di, 0] => [0, di], [0, dj] => [dj, 0], _ => unreachable!() },
				'|' if dj == 0 => {}
				'-' if di == 0 => {}
				'|' if dj != 0 => {
					beam(geometry, [i-1, j], [-1, 0], energy);
					beam(geometry, [i+1, j], [1, 0], energy);
					return;
				}
				'-' if di != 0 => {
					beam(geometry, [i, j-1], [0, -1], energy);
					beam(geometry, [i, j+1], [0, 1], energy);
					return;
				}
				_ => unreachable!(),
			}
			[i,j] = [i+di, j+dj];
		}
	}
	beam(geometry, [0, 0], [0, 1], energy);
	use itertools::Itertools;
	println!("{}", energy.iter().format_with("\n", |e,f| f(&e.iter().format_with("", |&e,f| f(&if e>0 {'#'}else{'.'})))));
	energy.into_iter().map(|row| row.into_iter()).flatten().filter(|&&e| e > 0).count()
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
		assert_eq!(result, Some(46));
	}

	#[test]
	fn test_part_two() {
		let result = part_two(&advent_of_code::template::read_file("examples", DAY));
		assert_eq!(result, None);
	}
}
