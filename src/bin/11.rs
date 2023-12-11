advent_of_code::solution!(11);

#[fehler::throws(as Option)] pub fn part_one(input: &str) -> usize {
	let space = input.lines().map(|row| row.chars().collect::<Box<_>>()).collect::<Box<_>>();
	fn expand(space: &[Box<[char]>]) -> Box<[Box<[char]>]> { space.iter().map(|line| std::iter::repeat(line.clone()).take(if line.iter().all(|&c| c=='.') { 2 } else { 1 })).flatten().collect::<Box<_>>() }
	fn transpose(space: &[Box<[char]>]) -> Box<[Box<[char]>]> { (0..space[0].len()).map(move |j| (0..space.len()).map(move |i| space[i][j]).collect::<Box<_>>()).collect::<Box<_>>() }
	let space = expand(&*transpose(&*expand(&*space)));
	use itertools::Itertools;
	println!("\n{}", space.iter().format_with("\n", |row, f| f(&row.iter().collect::<String>())));
	let galaxies = space.iter().enumerate().map(|(i, row)| row.iter().enumerate().filter_map(move |(j, &c)| (c=='#').then_some((i,j)))).flatten().collect::<Box<_>>();
	println!("{:?}", galaxies.iter().format(" "));
	galaxies.iter().tuple_combinations().map(|(&(ax,ay),&(bx,by))| ax.abs_diff(bx)+ay.abs_diff(by)).sum::<usize>()
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
		assert_eq!(result, Some(374));
	}

	#[test]
	fn test_part_two() {
		let result = part_two(&advent_of_code::template::read_file("examples", DAY));
		assert_eq!(result, Some(8410));
	}
}