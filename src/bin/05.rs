#![feature(iter_array_chunks)]
advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
	let mut input = input.split("\n\n");
	let seeds = input.next().unwrap().split_once(':').unwrap().1.split_whitespace().map(|s| s.parse::<u32>().unwrap());
	struct Range {destination_start: u32, source_start: u32, len: u32}
	//struct Map<'t> {source: &'t str, destination: &'t str, ranges: Box<[Range]>}
	//type Map = Box<[Range]>;
	let maps = input.map(|map| {
		/*let (source, map) = map.split_once('-').unwrap();
		let map = map.strip_prefix("-to-").unwrap();
		let (destination, map) = map.split_once(' ').unwrap();
		let map = map.strip_prefix(" map:\n").unwrap();*/
		let (_, map) = map.split_once(" map:\n").unwrap(); // Skips map keys parsing and assumes correct map order
		map.lines().map(|line| {
			let mut fields = line.split_whitespace().map(|s| s.parse().unwrap());
			Range{destination_start: fields.next().unwrap(), source_start: fields.next().unwrap(), len: fields.next().unwrap()}
		}).collect::<Box<_>>()
	}).collect::<Box<_>>();
	seeds.map(|mut seed| {
		for map in &*maps {
			let mut map = map.iter().filter_map(|Range{source_start, len, destination_start}|
				(*source_start <= seed && seed < source_start+len).then(|| { let next = seed-source_start+destination_start; println!("{seed} => {next}"); next})
			);
			let next = map.next().unwrap_or(seed);
			assert!(map.next().is_none());
			seed = next;
		}
		seed
	}).reduce(std::cmp::min)
}

pub fn part_two(input: &str) -> Option<u32> {
	let mut input = input.split("\n\n");
	let seeds = input.next().unwrap().split_once(':').unwrap().1.split_whitespace().map(|s| s.parse::<u32>().unwrap()).array_chunks().map(|[start, len]| start..start+len).flatten();
	let total = seeds.clone().count();
	println!("Reverse map would be {}x faster", total/125742456);
	struct Range {destination_start: u32, source_start: u32, len: u32}
	let maps = input.map(|map| {
		let (_, map) = map.split_once(" map:\n").unwrap(); // Skips map keys parsing and assumes correct map order
		map.lines().map(|line| {
			let mut fields = line.split_whitespace().map(|s| s.parse().unwrap());
			Range{destination_start: fields.next().unwrap(), source_start: fields.next().unwrap(), len: fields.next().unwrap()}
		}).collect::<Box<_>>()
	}).collect::<Box<_>>();
	let start = std::time::Instant::now();
	let mut last_status = std::time::Instant::now();
	let mut done = 0;
	seeds.map(|mut seed| {
		done += 1;
		if last_status.elapsed().as_secs() >= 1 {
			println!("Will be done within the next {} seconds", start.elapsed().as_secs() as u128*(total-done) as u128/done as u128);
			last_status = std::time::Instant::now();
		}
		//println!("{seed}");
		for map in &*maps {
			let mut map = map.iter().filter_map(|Range{source_start, len, destination_start}| (*source_start <= seed && seed < source_start+len).then(|| {
				let next = seed-source_start+destination_start;
				//println!("{seed} => {next}");
				next
			}));
			let next = map.next().unwrap_or(seed);
			//assert!(map.next().is_none());
			seed = next;
		}
		seed
	}).reduce(std::cmp::min)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_part_one() {
		let result = part_one(&advent_of_code::template::read_file("examples", DAY));
		assert_eq!(result, Some(35));
	}

	#[test]
	fn test_part_two() {
		let result = part_two(&advent_of_code::template::read_file("examples", DAY));
		assert_eq!(result, Some(46));
	}
}
