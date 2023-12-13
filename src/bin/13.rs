advent_of_code::solution!(13);

#[fehler::throws(as Option)] pub fn part_one(input: &str) -> usize {
	input.split("\n\n").map(|image| {
		let image = image.lines().map(|line| line.chars().collect::<Box<_>>()).collect::<Box<_>>();
		let row = || -> usize {
			'next_row: for i in 1..image.len() {
				let di = std::cmp::min(i, image.len()-i);
				for di in 1..=di { if image[i-di] != image[i+di-1] { continue 'next_row; } }
				return i;
			}
			return 0;
		}();
		let column = || -> usize {
			'next_column: for j in 1..image[0].len() {
				let dj = std::cmp::min(j, image[0].len()-j);
				for dj in 1..=dj { for row in &*image { if row[j-dj] != row[j+dj-1] { continue 'next_column; } } }
				return j;
			}
			return 0;
		}();
		row*100 + column
	}).sum()
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
		assert_eq!(result, Some(405));
	}

	#[test]
	fn test_part_two() {
		let result = part_two(&advent_of_code::template::read_file("examples", DAY));
		assert_eq!(result, Some(400));
	}
}
