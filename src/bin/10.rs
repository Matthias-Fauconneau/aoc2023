advent_of_code::solution!(10);

#[fehler::throws(as Option)] pub fn part_one(input: &str) -> u32 {
	let ref map = input.lines().map(|line| line.chars().map(|c| match c {
		'.' => ' ',
		'F' => '┌',
		'7' => '┐',
		'-' => '─',
		'|' => '│',
		'L' => '└',
		'J' => '┘',
		'S' => '┼',
		_ => unreachable!(),
	}).collect::<Box<_>>()).collect::<Box<_>>();
	use itertools::Itertools;
	fn format(map:&Box<[Box<[char]>]>) -> impl std::fmt::Display+'_ { map.iter().format_with("\n",|row,f| f(&row.iter().format_with("",|e,f| f(e)))) }
	println!("{}", format(map));
	let start = map.iter().enumerate().find_map(|(y,row)| row.iter().enumerate().find_map(|(x,&t)| (t=='┼').then_some(x)).map(|x| (x as i32,y as i32))).unwrap();
	fn index(image: &Box<[Box<[char]>]>, (x, y): (i32, i32)) -> char { image.get(y as usize).and_then(|row:&Box<[char]>| ((x as usize) < row.len()).then(|| row[x as usize])).unwrap_or(' ') }
	fn index_mut(image: &mut Box<[Box<[char]>]>, (x, y): (i32, i32)) -> &mut char { image.get_mut(y as usize).and_then(|row:&mut Box<[char]>| ((x as usize) < row.len()).then(|| &mut row[x as usize])).unwrap() }
	let connectivity = |t| match t {
		// N, W, E, S
		' ' => [false, false, false, false],
		'┌' => [false, false, true, true],
		'┐' => [false, true, false, true],
		'─' => [false, true, true, false],
		'│' => [true, false, false, true],
		'└' => [true, false, true, false],
		'┘' => [true, true, false, false],
		'┼' => [true, true, true, true],
		_ => unreachable!(),
	};
	let sewn = |[n,w,e,s]:[_;4]| [s,e,w,n];
	let neighbours = |(x,y)| [(x,y-1), (x-1,y), (x+1,y), (x,y+1)];
	fn from_iter<T, const N: usize>(mut iter: impl Iterator<Item=T>) -> [T; N] { let a = [(); N].map(|_| iter.next().unwrap()); assert!(iter.next().is_none()); a }
	let next = |center| {
		let neighbours = neighbours(center);
		let connections = neighbours.iter().enumerate().map(|(direction, &neighbour)| sewn(connectivity(index(map, neighbour)))[direction]);
		let center = connectivity(index(map, center));
		from_iter(center.iter().zip(connections).zip(neighbours).filter_map(|((&center, connection),neighbour)| (center && connection).then_some(neighbour)))
	};
	struct Walker { previous: (i32, i32), current: (i32, i32) }
	let mut walkers : [_; 2] = from_iter([start; 2].iter().zip(next(start)).map(|(&previous, current)| Walker{previous, current}));
	let mut steps = 1;
	let mut trace = map.clone();
	while walkers[0].current != walkers[1].current {
		for &Walker{current, ..} in &walkers { let c = index_mut(&mut trace, current); *c = match c {
			'┌' => '╔',
			'┐' => '╗',
			'─' => '═',
			'│' => '║',
			'└' => '╚',
			'┘' => '╝',
			'┼' => '╬',
			_ => unreachable!("\n{}", format(&trace)),
		}}
		walkers = walkers.map(|Walker{previous, current}| Walker{previous: current, current: match next(current) {
			[x, next] if x==previous => next,
			[next, x] if x==previous => next,
			_ => unreachable!(),
		}});
		steps += 1;
	}
	println!("{}", format(&trace));
	steps
}

#[fehler::throws(as Option)]  pub fn part_two(input: &str) -> usize {
	type Map = Box<[Box<[char]>]>;
	let ref map : Map = input.lines().map(|line| line.chars().map(|c| match c {
		'.' => '.',
		'F' => '┌',
		'7' => '┐',
		'-' => '─',
		'|' => '│',
		'L' => '└',
		'J' => '┘',
		'S' => '┼',
		_ => unreachable!(),
	}).collect()).collect();
	use itertools::Itertools;
	fn format(map: &Map) -> impl std::fmt::Display+'_ { map.iter().format_with("\n",|row,f| f(&row.iter().format_with("",|e,f| f(e)))) }
	println!("{}", format(map));
	let start = map.iter().enumerate().find_map(|(y,row)| row.iter().enumerate().find_map(|(x,&t)| (t=='┼').then_some(x)).map(|x| (x as i32,y as i32))).unwrap();
	fn index(image: &Map, (x, y): (i32, i32)) -> char { image.get(y as usize).and_then(|row:&Box<[char]>| ((x as usize) < row.len()).then(|| row[x as usize])).unwrap_or(' ') }
	fn index_mut(image: &mut Map, (x, y): (i32, i32)) -> &mut char { image.get_mut(y as usize).and_then(|row:&mut Box<[char]>| ((x as usize) < row.len()).then(|| &mut row[x as usize])).unwrap() }
	let connectivity = |t| match t {
		// N, W, E, S
		' '|'.' => [false, false, false, false],
		'┌' => [false, false, true, true],
		'┐' => [false, true, false, true],
		'─' => [false, true, true, false],
		'│' => [true, false, false, true],
		'└' => [true, false, true, false],
		'┘' => [true, true, false, false],
		'┼' => [true, true, true, true],
		c => unreachable!("{c}"),
	};
	let sewn = |[n,w,e,s]:[_;4]| [s,e,w,n];
	fn neighbours((x,y):(i32,i32)) -> [(i32,i32); 4] { [(x,y-1), (x-1,y), (x+1,y), (x,y+1)] }
	fn from_iter<T, const N: usize>(mut iter: impl Iterator<Item=T>) -> [T; N] { let a = [(); N].map(|_| iter.next().unwrap()); assert!(iter.next().is_none()); a }
	let next = |center| {
		let neighbours = neighbours(center);
		let connections = neighbours.iter().enumerate().map(|(direction, &neighbour)| sewn(connectivity(index(map, neighbour)))[direction]);
		let center = connectivity(index(map, center));
		from_iter(center.iter().zip(connections).zip(neighbours).filter_map(|((&center, connection),neighbour)| (center && connection).then_some(neighbour)))
	};
	struct Walker { previous: (i32, i32), current: (i32, i32) }
	let mut walkers : [_; 2] = from_iter([start; 2].iter().zip(next(start)).map(|(&previous, current)| Walker{previous, current}));
	let ref mut trace = map.clone();
	let step = |trace: &mut _, p| { let c = index_mut(trace, p); *c = match c {
		'┌' => '╔',
		'┐' => '╗',
		'─' => '═',
		'│' => '║',
		'└' => '╚',
		'┘' => '╝',
		'┼' => '╬',
		_ => unreachable!("\n{}", format(trace)),
	}};
	step(trace, start);
	while walkers[0].current != walkers[1].current {
		for &Walker{current, ..} in &walkers { step(trace, current); }
		walkers = walkers.map(|Walker{previous, current}| Walker{previous: current, current: match next(current) {
			[x, next] if x==previous => next,
			[next, x] if x==previous => next,
			_ => unreachable!(),
		}});
	}
	step(trace, walkers[0].current);
	println!("{}", format(trace));
	let ref mut expand : Map = std::iter::repeat(std::iter::repeat(' ').take(trace[0].len()*3).collect()).take(trace.len()*3).collect();
	for y in 0..trace.len() { for x in 0..trace[y].len() {
		let c = match trace[y][x] {
			'╔' => ['.','.','.', '.','╔','═', '.','║','.'],
			 '╗' => ['.','.','.', '═','╗','.', '.','║','.'],
			 '╚' => ['.','║','.', '.','╚','═', '.','.','.'],
			 '╝' => ['.','║','.', '═','╝','.', '.','.','.'],
			 '╬' => ['.','║','.', '═','╬','═', '.','║','.',],
			'═' => ['.','.','.', '═','═','═', '.','.','.'],
			'║' => ['.','║','.', '.','║','.', '.','║','.'],
			c if  " .┌┐─│└┘".contains(c) => ['.','.','.', '.','.','.', '.','.','.'],
			_ => unreachable!(),
		};
		let [x,y] = [x as i32, y as _];
		for ((dy, dx), c) in (0..3).cartesian_product(0..3).zip(c) { *index_mut(expand, (x*2+dx, y*2+dy)) = c; }
	}}
	let trace = expand;
	println!("{}", format(trace));
	fn walk(trace: &mut Map, position: (i32, i32)) {
		*index_mut(trace, position) = ' ';
		for neighbour in neighbours(position) {
			match index(trace, neighbour) {
				'.'|'┌'|'┐'|'─'|'│' |'└'|'┘' => walk(trace, neighbour),
				' '|'╔'|'╗'|'═'|'║'|'╚'|'╝'|'╬' => {},
				_ => unreachable!(),
			}
		}
	}
	walk(trace, (0,0));
	println!("{}", format(trace));
	let ref trace : Map = trace.iter().skip(1).step_by(2).map(|row| row.iter().skip(1).step_by(2).copied().collect()).collect();
	println!("{}", format(trace));
	trace.iter().map(|row| row.iter()).flatten().filter(|&&c| c == '.').count()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_part_one() {
		let result = part_one(&advent_of_code::template::read_file("examples", DAY));
		assert_eq!(result, Some(8));
	}

	#[test]
	fn test_part_two() {
		let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 0));
		assert_eq!(result, Some(4));
		let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 1));
		assert_eq!(result, Some(4));
		let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
		assert_eq!(result, Some(10));
	}
}
