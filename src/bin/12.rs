advent_of_code::solution!(12);

#[fehler::throws(as Option)] pub fn part_one(input: &str) -> usize {
	input.lines().map(|line| {
		let (springs, damaged) = line.split_once(' ').unwrap();
		let damaged = damaged.split(',').map(|n| n.parse::<usize>().unwrap()).collect::<Box<_>>();
		let unknown = springs.chars().filter(|&c| c == '?').count();
		//println!("{springs} {damaged:?} {unknown}");
		(0..(1<<unknown)).map(|candidate| {
			let mut unknown_index = 0;
			springs.chars().map(move |c| match c {
				'.' => ' ',
				'#' => '#',
				'?' => { let c = if candidate & (1<<unknown_index) == 0 { ' ' } else { '#' }; unknown_index += 1; c }
				_ => unreachable!()
			}).collect::<String>()
		}).filter(|row| {
			let row = row.split_whitespace().map(|group| group.len()).collect::<Box<_>>();
			&*row == &*damaged
		}).count()
	}).sum()
}

#[fehler::throws(as Option)]  pub fn part_two(input: &str) -> usize {
	let lines = input.lines().collect::<Box<_>>();
	println!("");
	lines.iter().enumerate().map(|(i,line)| {
		let (springs, damage) = line.split_once(' ').unwrap();
		let springs = std::iter::repeat(springs).take(5).collect::<Box<_>>().join("?");
		let damage = std::iter::repeat(damage).take(5).collect::<Box<_>>().join(",");
		let damage = damage.split(',').map(|n| n.parse::<usize>().unwrap()).collect::<Box<_>>();

		let unknowns = springs.chars().enumerate().filter_map(|(i, c)| (c == '?').then_some(i)).collect::<Box<_>>();
		println!("{i}/{} {springs} {damage:?} {}", lines.len(), unknowns.len());

		let mut springs = springs.chars().map(|c| match c {'#'|'?'=> false, '.'=>true, _=>unreachable!()}).collect::<Box<_>>();
		//let mut springs : u128 = springs.chars().map(|c| match c {'#'|'?'=> false, '.'=>true, _=>unreachable!()}).enumerate().fold(0, |mask, (i, state)| mask|(if state{1}else{0}<<i));
		let mut index = unknowns.iter().map(|_| 0).collect::<Box<_>>();
		let mut count = 0;
		'next: loop {
			// Increment
			let mut i = unknowns.len()-1; // msb first: to reduce prefix invalidations
			loop {
				if index[i] == 0 { index[i]=1; springs[unknowns[i]] = true; break; }
				else { // carry
					index[i] = 0; springs[unknowns[i]] = false;
					if i == 0 { return count; } // carry over msb => overflow => all checked
					i -= 1;
				}
			}

			let mut i = 0;
			for &damage in &*damage {
				while springs[i] { i+=1; }
				let start = i;
				while i<springs.len() && !springs[i] { i+=1; }
				if i-start != damage { continue 'next; }
			}
			while i<springs.len() && springs[i] { i+=1; }
			if i != springs.len() { continue 'next; }
			count += 1;
		}

		/*let max_gap = springs.split('#').map(|s| s.len()).max().unwrap();
		assert!(max_gap <= 13, "{springs} {max_gap}");
		let max_damage = *damage.iter().max().unwrap();
		assert!(max_damage <= 6, "{max_damage}");

		let total_damage = damage.iter().sum::<usize>();
		if total_damage >= springs.len() { println!("{} {}", springs.len(), total_damage); return 0; }

		let total_ok_or_unknown = springs.len() - total_damage;
		let max_total_gap = total_ok_or_unknown;

		let maybe_ok : u128 = springs.chars().map(|c| match c {'#'=> false, '.'|'?'=>true, _=>unreachable!()}).enumerate().fold(0, |mask, (i, state)| mask|(if state{1}else{0}<<i));
		let maybe_damage : u128 = springs.chars().map(|c| match c {'.'=> false, '#'|'?'=>true, _=>unreachable!()}).enumerate().fold(0, |mask, (i, state)| mask|(if state{1}else{0}<<i));

		let mut gaps = damage.iter().map(|_| 1).collect::<Box<_>>();
		gaps[0] = 0;
		let mut total_gap = gaps.iter().sum::<usize>();
		let mut count = 0;
		println!("{i}/{} {springs} {damage:?} {max_gap}", lines.len());
		'count: loop {
			(|| {
				let [mut maybe_ok, mut maybe_damage] = [maybe_ok, maybe_damage];
				const MASK : [u64; 7]= [0, 0b1, 0b11, 0b111, 0b1111, 0b1_1111, 0b11_1111];//, 0b111_1111];
				for (&gap, &damage) in gaps.iter().zip(&*damage) {
					if (maybe_ok as u64)&MASK[gap] != MASK[gap] { return; }
					maybe_ok >>= gap; maybe_damage >>= gap;
					if (maybe_damage as u64)&MASK[damage] != MASK[damage] { return; }
					maybe_ok >>= damage; maybe_damage >>= damage;
				}
				let remain = springs.len() - total_damage - total_gap;
				println!("\n{springs}");
				use itertools::Itertools;
				println!("{}", gaps.iter().zip(&*damage).map(|(&gap, &damage)| ".".repeat(gap)+&"#".repeat(damage)).format(""));
				if remain >=6 { println!("{remain}"); return; }
				if (maybe_ok as u64)&MASK[remain] != MASK[remain] { return; }
				/*let mut springs = springs.chars();
				for (&gap, &damage) in gaps.iter().zip(&*damage) {
					for _ in 0..gap { assert!(".?".contains(springs.next().unwrap())); }
					for _ in 0..damage { assert!("#?".contains(springs.next().unwrap())); }
				}*/
				count += 1;
			})();

			// Increment gaps
			let mut i = gaps.len()-1; // end is least significant to reduce prefix invalidations // TODO FIXME: not doing anything with prefix right now
			loop {
				if gaps[i] < max_gap && total_gap<max_total_gap { gaps[i]+=1; total_gap+=1; break; }
				else { // carry
					total_gap -= gaps[i]-1;
					gaps[i] = 1;
					if i == 0 { break 'count; } // carry over msb => overflow => all checked
					i -= 1;
				}
			}
		}*/
		//println!("{i}/{} {springs} {damage:?} {count}", lines.len());
		println!("{count}");
		count
	}).sum()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_part_one() {
		let result = part_one(&advent_of_code::template::read_file("examples", DAY));
		assert_eq!(result, Some(21));
	}

	#[test]
	fn test_part_two() {
		let result = part_two(&advent_of_code::template::read_file("examples", DAY));
		assert_eq!(result, Some(525152));
	}
}
