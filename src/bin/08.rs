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
	let [start, end] = ['A', 'Z'].map(|label| nodes.iter().enumerate().filter_map(|(i, &(k,_))| k.ends_with(label).then_some(i)).collect::<Box<_>>());
	let nodes = nodes.iter().map(|(_, nexts)| nexts.map(|next| nodes.iter().position(|&(k,_)| k==next).unwrap())).collect::<Box<_>>();
	let mut position = start;
	let mut count = 0;
	let mut trace = Vec::new();
	let start = std::time::Instant::now();
	let mut last_status = std::time::Instant::now();
	println!("Ghotsts:{} × Sequence: {} × Nodes: {} = {}", position.len(), sequence.chars().count(), nodes.len(), position.len() * sequence.chars().count() * nodes.len());
	for (_step_index, step) in sequence.chars().enumerate().cycle() {
		/*if false {
			let id = (_step_index, position.clone());
			assert!(!trace.contains(&id));
			trace.push(id);
		}*/ trace.push(());
		if last_status.elapsed().as_secs() >= 1 {
			let total = position.len() * sequence.chars().count() * nodes.len();
			let done = trace.len();
			println!("{}K/{}K={:.0} Will be done within the next {} seconds", done/1000, total/1000, done as f32 / total as f32 * 100., start.elapsed().as_secs() as u128*(total-done) as u128/done as u128);
			last_status = std::time::Instant::now();
		}
		//println!("{step_index} {step} {position:?}");
		for position in position.iter_mut() { *position = nodes[*position][match step { 'L' => 0, 'R' => 1, _ => unreachable!()}] }
		count += 1;
		if position.iter().all(|position| end.contains(position)) { break; }
	}
	count
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
