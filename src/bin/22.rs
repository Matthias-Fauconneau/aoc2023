#![feature(anonymous_lifetime_in_impl_trait)]
advent_of_code::solution!(22);

#[fehler::throws(as Option)] pub fn part_one(input: &str) -> usize {
	let mut bricks = input.lines().map(|line| {
		let (min, max) = line.split_once('~').unwrap();
		let [min, max] = [min, max].map(|xyz| <[_; 3]>::try_from(xyz.split(',').map(|coordinate| coordinate.parse::<i32>().unwrap()).collect::<Vec<_>>()).unwrap());
		let min = min.map(|c| c-1);
		let max = max.map(|c| c-1); // -1 to zero-index, +1 to exclusive end?
		[min, max]
	}).collect::<Box<_>>();
	type Brick = [[i32; 3]; 2];
	fn fall(bricks: impl IntoIterator<Item=&Brick> , brick: &Brick) -> Option<Brick> {
		let fallen@[fallen_min, fallen_max] = brick.map(|p| [p[0], p[1], p[2]-1]);
		(!bricks.into_iter().any(|[min, max]| {
			let min : [_; 3] = std::array::from_fn(|i| min[i].max(fallen_min[i]));
			let max : [_; 3] = std::array::from_fn(|i| max[i].min(fallen_max[i]));
			min <= max
		})).then_some(fallen)
	}
	loop {
		let mut unstable = false;
		for i in 0..bricks.len() {
			if let Some(fallen) = fall(&*bricks, &bricks[i]) {
				bricks[i] = fallen;
				unstable = true;
			}
		}
		if !unstable { break; }
	}
	bricks.iter().filter(|except| bricks.iter().any(|brick| fall(bricks.iter().filter(|brick| brick!=except), brick).is_none())).count()
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
		assert_eq!(result, Some(5));
	}

	#[test]
	fn test_part_two() {
		let result = part_two(&advent_of_code::template::read_file("examples", DAY));
		assert_eq!(result, None);
	}
}
