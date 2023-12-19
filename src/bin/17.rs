#![feature(array_chunks)]
advent_of_code::solution!(17);

pub fn part_one(input: &str) -> Option<u32> {
	let ref heat_loss = input.lines().map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect::<Box<_>>()).collect::<Box<_>>();
	fn walk(heat_loss: &[Box<[u32]>], [previous_i, previous_j]: [isize; 2], [di, dj]: [isize; 2], path_heat_loss: u32, ref mut trace: Box<[bool]>) -> Option<u32> {
		let [next_i, next_j] = [previous_i+di, previous_j+dj];
		if next_i<0 || next_i>=heat_loss.len() as _ || next_j<0 || next_j>=heat_loss[0].len() as _ { return None; }
		let path_heat_loss = path_heat_loss + match [di,dj] {
			[di, 0] => (if di < 0 {di..0} else {1..di+1}).map(|di| heat_loss[(previous_i+di) as usize][previous_j as usize]).sum::<u32>(),
			[0, dj] => (if dj < 0 {dj..0} else {1..dj+1}).map(|dj| heat_loss[previous_i as usize][(previous_j+dj) as usize]).sum(),
			_ => unreachable!()
		};
		static BEST : std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(u32::MAX);
		if BEST.load(std::sync::atomic::Ordering::Relaxed) <= path_heat_loss { return None; }
		let columns = heat_loss[0].len();
		if [next_i as usize, next_j as _] == [heat_loss.len()-1, heat_loss[0].len()-1] {
			if BEST.fetch_min(path_heat_loss, std::sync::atomic::Ordering::Relaxed) > path_heat_loss { println!("{path_heat_loss}"); }
			use itertools::Itertools;
			println!("{}\n", trace.chunks_exact(columns).format_with("\n", |e,f| f(&e.iter().map(|&trace| if trace{'x'}else{'.'}).format(""))));
			return Some(path_heat_loss)
		}
		match [di,dj] {
			[di, 0] => for di in if di < 0 {di..0} else {1..di+1} {
				let ref mut trace = trace[((previous_i+di) as usize)*columns+next_j as usize];
				if *trace { return None; }
				*trace = true;
			},
			[0, dj] => for dj in if dj < 0 {dj..0} else {1..dj+1} {
				let ref mut trace = trace[(next_i as usize)*columns+(previous_j+dj) as usize];
				if *trace { return None; }
				*trace = true;
			}
			_ => unreachable!()
		};

		match [di,dj] {[_,0] => {[[0,-3],[0,-2],[0,-1],[0,1],[0,2],[0,3]]}, [0,_] => {[[-3,0],[-2,0],[-1,0],[1,0],[2,0],[3,0]]}, _=>unreachable!()}
			.iter().filter_map(|&[di,dj]| walk(heat_loss, [next_i, next_j], [di, dj], path_heat_loss, trace.clone())).min()
	}
	[[1,0],[0,1]].iter().filter_map(|&direction| walk(heat_loss, [0,0], direction, 0, heat_loss.into_iter().map(|row| row.into_iter().map(|_| false)).flatten().collect::<Box<_>>())).min()
}

#[fehler::throws(as Option)] pub fn part_two(_input: &str) -> u32 {
	fehler::throw!();
}

#[cfg(test)]
mod tests {
	use super::*;

	/*#[test]
	fn test_part_one() {
		let result = part_one(&advent_of_code::template::read_file("examples", DAY));
		assert_eq!(result, Some(102));
	}*/

	#[test]
	fn test_part_two() {
		let result = part_two(&advent_of_code::template::read_file("examples", DAY));
		assert_eq!(result, None);
	}
}
