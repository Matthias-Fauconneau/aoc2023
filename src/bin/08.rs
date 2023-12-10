advent_of_code::solution!(8);

#[fehler::throws(as Option)] pub fn part_one(input: &str) -> usize {
	let (sequence, map) = input.split_once("\n\n").unwrap();
	let nodes = map.lines().map(|line| {
		let (key, next) = line.split_once('=').unwrap();
		let key = key.trim();
		let (left, right) = next.trim().strip_prefix('(').unwrap().strip_suffix(')').unwrap().split_once(", ").unwrap();
		(key, [left, right])
	}).collect::<Box<_>>();
	let [start, end] = ["AAA","ZZZ"].map(|label| nodes.iter().position(|&(k,_)| k==label).unwrap());
	let nodes = nodes.iter().map(|(_, nexts)| nexts.map(|next| nodes.iter().position(|&(k,_)| k==next).unwrap())).collect::<Box<_>>();
	sequence.chars().cycle().scan(start, |position, step| {
		*position = nodes[*position][match step { 'L' => 0, 'R' => 1, _ => unreachable!()}];
		Some(*position)
	}).position(|position| position == end).unwrap()+1
}

#[fehler::throws(as Option)]  pub fn part_two(input: &str) -> usize {
	let (sequence, map) = input.split_once("\n\n").unwrap();
	let nodes = map.lines().map(|line| {
		let (key, next) = line.split_once('=').unwrap();
		let key = key.trim();
		let (left, right) = next.trim().strip_prefix('(').unwrap().strip_suffix(')').unwrap().split_once(", ").unwrap();
		(key, [left, right])
	}).collect::<Box<_>>();
	fn from_iter_or_default<T:Default, const N: usize>(mut iter: impl Iterator<Item=T>) -> [T; N] { std::array::from_fn(|_| iter.next().unwrap_or_default()) }
	let [start, end] = ['A', 'Z'].map(|label| from_iter_or_default::<_, 6>(nodes.iter().enumerate().filter_map(|(i, &(k,_))| k.ends_with(label).then_some(Some(i)))));
	//let ghosts = start.iter().filter_map(|&o| o).count();
	println!("{start:?} {end:?}");
	let [start, _end] = [start, end].map(|p| p.map(|o| o.unwrap_or(p[0].unwrap())));
 	let nodes = nodes.iter().map(|(_, nexts)| nexts.map(|next| nodes.iter().position(|&(k,_)| k==next).unwrap())).collect::<Box<_>>();
	let mut position = start;
	let mut trace = [(); 6].map(|_| Vec::new());
	println!("Ghosts:{} × Sequence: {} × Nodes: {} = {}", position.len(), sequence.chars().count(), nodes.len(), position.len() * sequence.chars().count() * nodes.len());
	let mut ghost_loop_len = [0; 6];
	for (step_id, step) in sequence.chars().enumerate().cycle() {
		for ((position, trace), ghost_loop_len) in position.iter().zip(&mut trace).zip(&mut ghost_loop_len) {
			if *ghost_loop_len == 0 {
				let id = (step_id, *position);
				if trace.contains(&id) {
					*ghost_loop_len = trace.len() - trace.iter().position(|&t| t==id).unwrap();
					println!("{}", ghost_loop_len);
				} else {
					trace.push(id);
				}
			}
		}
		if ghost_loop_len.iter().all(|&len|len > 0) { break; }
		println!("{step_id} {step} {position:?}");
		for position in position.iter_mut() { *position = nodes[*position][match step { 'L' => 0, 'R' => 1, _ => unreachable!()}] }
	}
	println!("{ghost_loop_len:?}");
	fn gcd(mut a: usize, mut b: usize) -> usize { while b != 0 { (a,b) = (b, a % b) } a }
	ghost_loop_len.into_iter().reduce(|a, b| a*b/gcd(a,b)).unwrap()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_part_one() {
		let result = part_one(&advent_of_code::template::read_file("examples", DAY));
		assert_eq!(result, Some(6));
	}

	#[test]
	fn test_part_two() {
		let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
		assert_eq!(result, Some(6));
	}
}
