advent_of_code::solution!(16);

#[fehler::throws(as Option)] pub fn part_one(input: &str) -> usize {
	let ref geometry = input.lines().map(|line| line.chars().collect::<Box<_>>()).collect::<Box<_>>();
	let ref mut beams = geometry.iter().map(|line| line.iter().map(|_| Vec::new()).collect::<Box<_>>()).collect::<Box<_>>();
	fn beam(geometry: &[Box<[char]>], [mut i, mut j]: [isize; 2], [mut di, mut dj]: [isize; 2], ref mut beams: &mut [Box<[Vec<[isize; 2]>]>]) {
		loop {
			if i < 0  || i >= geometry.len() as isize || j<0 || j >= geometry[0].len() as isize || beams[i as usize][j as usize].contains(&[di,dj]) { return; }
			beams[i as usize][j as usize].push([di,dj]);
			match geometry[i as usize][j as usize] {
				'.' => {},
				'/' => [di, dj] = match [di, dj] { [di, 0] => [0, -di], [0, dj] => [-dj, 0], _ => unreachable!() },
				'\\' => [di, dj] = match [di, dj] { [di, 0] => [0, di], [0, dj] => [dj, 0], _ => unreachable!() },
				'|' if dj == 0 => {}
				'-' if di == 0 => {}
				'|' if dj != 0 => {
					beam(geometry, [i-1, j], [-1, 0], beams);
					beam(geometry, [i+1, j], [1, 0], beams);
					return;
				}
				'-' if di != 0 => {
					beam(geometry, [i, j-1], [0, -1], beams);
					beam(geometry, [i, j+1], [0, 1], beams);
					return;
				}
				_ => unreachable!(),
			}
			[i,j] = [i+di, j+dj];
		}
	}
	beam(geometry, [0, 0], [0, 1], beams);
	use itertools::Itertools;
	println!("\n{}", beams.iter().format_with("\n", |e,f| f(&e.iter().format_with("", |e,f| f(&match e[..] {
		[] => '.',
		[[di,dj]] => match [di,dj] {
			[-1,0] => '^',
			[1,0] => 'v',
			[0,-1] => '<',
			[0,1] => '>',
			_ => unreachable!(),
		},
		_ =>  char::from_digit(e.len() as u32, 36).unwrap(),
	})))));
	//println!("{}", beams.iter().format_with("\n", |e,f| f(&e.iter().format_with("", |e,f| f(&if e.is_empty() {'.'}else{'#'})))));
	beams.into_iter().map(|row| row.into_iter()).flatten().filter(|&e| !e.is_empty()).count()
}

#[fehler::throws(as Option)] pub fn part_two(input: &str) -> usize {
	let ref geometry = input.lines().map(|line| line.chars().collect::<Box<_>>()).collect::<Box<_>>();
	(0..geometry.len()).map(|i| ([i, 0], [0, 1])).chain(
	(0..geometry.len()).map(|i| ([i, geometry[0].len()-1], [0, -1])).chain(
	(0..geometry[0].len()).map(|j| ([0, j], [1, 0])).chain(
	(0..geometry[0].len()).map(|j| ([geometry.len()-1, j], [-1, 0]))
	)))
	.map(|([i,j],[di,dj])| {
		let ref mut beams = geometry.iter().map(|line| line.iter().map(|_| Vec::new()).collect::<Box<_>>()).collect::<Box<_>>();
		fn beam(geometry: &[Box<[char]>], [mut i, mut j]: [isize; 2], [mut di, mut dj]: [isize; 2], ref mut beams: &mut [Box<[Vec<[isize; 2]>]>]) {
			loop {
				if i < 0  || i >= geometry.len() as isize || j<0 || j >= geometry[0].len() as isize || beams[i as usize][j as usize].contains(&[di,dj]) { return; }
				beams[i as usize][j as usize].push([di,dj]);
				match geometry[i as usize][j as usize] {
					'.' => {},
					'/' => [di, dj] = match [di, dj] { [di, 0] => [0, -di], [0, dj] => [-dj, 0], _ => unreachable!() },
					'\\' => [di, dj] = match [di, dj] { [di, 0] => [0, di], [0, dj] => [dj, 0], _ => unreachable!() },
					'|' if dj == 0 => {}
					'-' if di == 0 => {}
					'|' if dj != 0 => {
						beam(geometry, [i-1, j], [-1, 0], beams);
						beam(geometry, [i+1, j], [1, 0], beams);
						return;
					}
					'-' if di != 0 => {
						beam(geometry, [i, j-1], [0, -1], beams);
						beam(geometry, [i, j+1], [0, 1], beams);
						return;
					}
					_ => unreachable!(),
				}
				[i,j] = [i+di, j+dj];
			}
		}
		beam(geometry, [i as isize, j as isize], [di,dj], beams);
		use itertools::Itertools;
		println!("\n{}", beams.iter().format_with("\n", |e,f| f(&e.iter().format_with("", |e,f| f(&match e[..] {
			[] => '.',
			[[di,dj]] => match [di,dj] {
				[-1,0] => '^',
				[1,0] => 'v',
				[0,-1] => '<',
				[0,1] => '>',
				_ => unreachable!(),
			},
			_ =>  char::from_digit(e.len() as u32, 36).unwrap(),
		})))));
		//println!("{}", beams.iter().format_with("\n", |e,f| f(&e.iter().format_with("", |e,f| f(&if e.is_empty() {'.'}else{'#'})))));
		beams.into_iter().map(|row| row.into_iter()).flatten().filter(|&e| !e.is_empty()).count()
	}).max().unwrap()
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
		assert_eq!(result, Some(51));
	}
}
