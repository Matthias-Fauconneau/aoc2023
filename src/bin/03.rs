advent_of_code::solution!(3);

#[fehler::throws(as Option)] pub fn part_one(input: &str) -> u32 {
	let rows = input.split('\n').collect::<Box<[&str]>>();
	let mut sum = 0;
	for (y, row) in rows.iter().enumerate() {
		let mut x = 0;
		while x < row.chars().count() {
			let is = |x: i32, y: i32, p:&dyn Fn(_)->bool| { rows.get(y as usize).map(|row: &&str| row.chars().nth(x as usize).map(p)).flatten().unwrap_or(false) };
			let digit = |x,y| { is(x as _,y as _, &Box::new(|c| c >= '0' && c <=  '9')) };
			let symbol = |x,y| { is(x,y, &Box::new(|c| c!='.' && !(c >= '0' && c <=  '9'))) };
			if digit(x, y) {
				let (number, _) = rows[y][x..].split_once(|c| !char::is_digit(c, 10)).unwrap_or((&rows[y][x..],""));
				let number : u32 = number.parse().unwrap();
				let mut any = false;
				while digit(x, y) {
					for dy in -1..=1 { for dx in -1..=1 {
						if symbol(x as i32 + dx, y as i32 + dy) { any = true; }
					}}
					x+=1;
				}
				if any { sum += dbg!(number); }
			} else {
				x+=1;
			}
		}
	}
	sum
}

#[fehler::throws(as Option)]  pub fn part_two(input: &str) -> u32 {
	let rows = input.split('\n').collect::<Box<[&str]>>();
	struct Gear { x: usize, y: usize, numbers: Vec<u32> }
	let mut gears = Vec::<Gear>::new();
	for (y, row) in rows.iter().enumerate() {
		let mut x = 0;
		while x < row.chars().count() {
			let is = |x: i32, y: i32, p:&dyn Fn(_)->bool| { rows.get(y as usize).map(|row: &&str| row.chars().nth(x as usize).map(p)).flatten().unwrap_or(false) };
			let digit = |x,y| { is(x as _,y as _, &Box::new(|c| c >= '0' && c <=  '9')) };
			let gear = |x,y| { is(x,y, &Box::new(|c| c=='*')) };
			if digit(x, y) {
				let (number, _) = rows[y][x..].split_once(|c| !char::is_digit(c, 10)).unwrap_or((&rows[y][x..],""));
				let number : u32 = number.parse().unwrap();
				while digit(x, y) {
					for dy in -1..=1 { for dx in -1..=1 {
						let [x,y] = [x as i32 + dx, y as i32 + dy];
						if gear(x, y) {
							let [x,y] = [x as _, y as _];
							let gear = if let Some(gear) = gears.iter_mut().find(|g| g.x == x && g.y==y) { gear } else { gears.push(Gear{x, y, numbers: Vec::new()}); gears.last_mut().unwrap() };
							if !gear.numbers.contains(&number) { gear.numbers.push(number); }
						}
					}}
					x+=1;
				}
			} else {
				x+=1;
			}
		}
	}
	//for gear in &gears { assert!(gear.numbers.len() <= 2); }
	gears.into_iter().filter_map(|Gear{numbers,..}| <[_;2]>::try_from(numbers).map(|[a,b]| a*b).ok()).sum()
	//gears.map(|Gear{numbers,..}| numbers.iter().product::<u32>()).sum()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_part_one() {
		let result = part_one(&advent_of_code::template::read_file("examples", DAY));
		assert_eq!(result, Some(4361));
	}

	#[test]
	fn test_part_two() {
		let result = part_two(&advent_of_code::template::read_file("examples", DAY));
		assert_eq!(result, Some(467835));
	}
}
