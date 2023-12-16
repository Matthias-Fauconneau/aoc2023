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

#[fehler::throws(as Option)]  pub fn part_two(input: &str) -> usize {
	input.split("\n\n").map(|image| {
		let ref mut image = image.lines().map(|line| line.chars().collect::<Box<_>>()).collect::<Box<_>>();
		fn row(image:&[Box<[char]>]) -> impl Iterator<Item=usize>+'_ {
			 (1..image.len()).filter_map(|i| {
				let di = std::cmp::min(i, image.len()-i);
				for di in 1..=di { if image[i-di] != image[i+di-1] { return None; } }
				Some(i)
			})
		}
		fn column(image:&[Box<[char]>]) -> impl Iterator<Item=usize>+'_ {
			(1..image[0].len()).filter_map(|j| {
				let dj = std::cmp::min(j, image[0].len()-j);
				for dj in 1..=dj { for row in &*image { if row[j-dj] != row[j+dj-1] { return None; } } }
				Some(j)
			})
		}
		let unsmudged = [row(image).next(), column(image).next()];
		use itertools::Itertools;
		for i in 0..image.len() { for j in 0..image[0].len() {
			let toggle = |c:&mut _| *c = match c { '#'=>'.', '.'=>'#', _ => unreachable!() };
			let smudged = {toggle(&mut image[i][j]); let f=[row(image).collect::<Box<_>>(), column(image).collect()]; toggle(&mut image[i][j]); f};
			let smudged = std::array::from_fn(|i| smudged[i].iter().copied().find(|&smudged| Some(smudged)!=unsmudged[i])); /*unsmudged reflection stays in smudged search if not affected by smudge (out of reflection range)*/
			if smudged != [None, None] {
				let [row, column] = smudged;
				let summary = row.unwrap_or(0)*100 + column.unwrap_or(0);
				println!("{}\n{summary}", image.iter().format_with("\n", |e,f| f(&e.iter().collect::<String>())));
				assert!(summary != 0, "{i} {j} {unsmudged:?} {smudged:?}");
				return summary;
			} else { continue; }
		}}
		panic!("{}\n{unsmudged:?}", image.iter().format_with("\n", |e,f| f(&e.iter().collect::<String>())));
	}).sum()
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
