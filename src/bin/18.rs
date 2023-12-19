#![feature(array_windows, get_many_mut)]#![allow(non_snake_case)]
advent_of_code::solution!(18);

pub fn part_one(input: &str) -> Option<usize> {
	let plan = input.lines().map(|line| { let [direction, length, _] = line.split(' ').collect::<Vec<_>>().try_into().unwrap(); (direction, length.parse::<isize>().unwrap()) }).collect::<Box<_>>();
	let [mut min, mut max, mut digger] = [[0; 2]; 3];
	for &(direction, length) in &*plan {
		match direction {
			"U" => digger[0] -= length,
			"D" => digger[0] += length,
			"L" => digger[1] -= length,
			"R" => digger[1] += length,
			_ => unreachable!()
		}
		for i in 0..2 {
			min[i] = min[i].min(digger[i]);
			max[i] = max[i].max(digger[i]);
		}
	}
	let [rows, columns] = std::array::from_fn(|i| (max[i]+1-min[i]) as usize);
	let mut trace = std::iter::repeat(false).take(rows*columns).collect::<Box<_>>();
	let mut digger = [-min[0],-min[1]];
	trace[digger[0] as usize*columns+digger[1] as usize] = true;
	for &(direction, length) in &*plan {
		for _ in 0..length {
			match direction {
				"U" => digger[0] -= 1,
				"D" => digger[0] += 1,
				"L" => digger[1] -= 1,
				"R" => digger[1] += 1,
				_ => unreachable!()
			}
			trace[digger[0] as usize*columns+digger[1] as usize] = true;
		}
	}
	use itertools::Itertools;
	println!("{}\n", trace.chunks_exact(columns).format_with("\n", |e,f| f(&e.iter().map(|&trace| if trace{'x'}else{'.'}).format(""))));
	fn walk([rows, columns]: [usize; 2], trace: &mut [bool], [i,j]: [isize; 2]) -> Option<()> {
        trace[i as usize*columns+j as usize] = true;
        for [di,dj] in [[0,-1],[-1,0],[1,0],[0,1]] {
			let [i,j] = [i+di, j+dj];
			if i<0 || i>=rows as _|| j<0 || j>=columns as _ { return None; }
			if !trace[i as usize*columns+j as usize] { walk([rows, columns], trace, [i,j])?; }
        }
		Some(())
    }
	for i in 0..rows { for j in 0..columns {
		let mut trace = trace.clone();
		if let Some(count) = std::thread::Builder::new().stack_size(rows*columns*256).spawn(move || {
			let ref mut trace = trace;
			walk([rows, columns], trace, [i as isize, j as _]).map(|_| trace.into_iter().filter(|&&x| x).count())
			// else seed was outside trace
		}).unwrap().join().unwrap() {
			return Some(count);
		}
	}}
	None
}

#[fehler::throws(as Option)] pub fn part_two(input: &str) -> usize {
	let plan = input.lines().map(|line| {
		let [_, _, color] = line.split(' ').collect::<Vec<_>>().try_into().unwrap();
		let color = color.strip_prefix("(#").unwrap().strip_suffix(')').unwrap();
		let (length, direction) = color.split_at(color.len()-1);
		(usize::from_str_radix(direction, 4).unwrap(), usize::from_str_radix(length, 16).unwrap())
	}).collect::<Box<_>>();
	let winding = plan.array_windows().map(|[(previous_direction, _),(next_direction, _)]| match [previous_direction, next_direction] {
		[0,1]|[1,2]|[2,3]|[3,0] => 1,
		[0,3]|[3,2]|[2,1]|[1,0] => -1,
		_ => unreachable!(),
	}).sum::<isize>();
	assert_eq!(winding, 3); // Clock wise (i.e turning around right ward)
	let mut polygon = plan.iter().scan([0,0], |digger, &(direction, length)| {
		match direction {
			0 => digger[1] += length as isize, // Right
			1 => digger[0] += length as isize, // Down
			2 => digger[1] -= length as isize, // Left
			3 => digger[0] -= length as isize, // Up
			_ => unreachable!()
		}
		Some(*digger)
	}).collect::<Vec<_>>();
	assert_eq!(*polygon.last().unwrap(), [0,0]);
	let mut area = 0;
	let mut i = 0;
	while polygon.len() > 4 { // Ear clipping
		let [a,b,c,d] = [i%polygon.len(), (i+1)%polygon.len(), (i+2)%polygon.len(), (i+3)%polygon.len()];
		let [A,B,C,D] = polygon.get_many_mut([a,b,c,d]).unwrap();
		fn cross([ai,aj]: [isize; 2], [bi,bj]: [isize; 2]) -> isize { ai*bj - bi*aj }
		fn sub([ai,aj]: [isize; 2], [bi,bj]: [isize; 2]) -> [isize; 2] { [ai-bi, aj-bj] }
		if cross(sub(*C,*A), sub(*B,*A)) > 0 && cross(sub(*D,*B), sub(*C,*B)) > 0 { // Convex
			let [di, dj] = match sub(*C,*B) {
				[di,0] if di > 0 => { let j = A[1].max(D[1]); assert_eq!(B[1], C[1]); let dj = B[1]-j; assert!(dj>0); B[1] = j; C[1] = j; [di, dj]}
				[di,0] if di < 0 => { let j = A[1].min(D[1]); assert_eq!(B[1], C[1]); let dj = j-B[1]; assert!(dj>0); B[1] = j; C[1] = j; [di, dj]}
				[0,dj] if dj > 0 => { let i = A[0].min(D[0]); assert_eq!(B[0], C[0]); let di = i-B[0]; assert!(di>0); B[0] = i; C[0] = i; [di, dj]}
				[0,dj] if dj < 0 => { let i = A[0].max(D[0]); assert_eq!(B[0], C[0]); let di = B[0]-i; assert!(di>0); B[0] = i; C[0] = i; [di, dj]}
				_ => unreachable!()
			};
			area += di.abs() as usize * dj.abs() as usize;
			let [A,B,C,D] = [*A,*B,*C,*D];
			if b < c {
				if C == D { println!("{a} {b} {c} {d} -{c} {polygon:?}"); polygon.remove(c); }
				if A == B { println!("{a} {b} {c} {d} -{b} {polygon:?}"); polygon.remove(b); }
			} else if c < b {
				if A == B { println!("{a} {b} {c} {d} -{b} {polygon:?}"); polygon.remove(b); }
				if C == D { println!("{a} {b} {c} {d} -{c} {polygon:?}"); polygon.remove(c); }
			} else { unreachable!(); }
			for i in i-1..=i+1 {
				let [a,b,c] = [i%polygon.len(), (i+1)%polygon.len(), (i+2)%polygon.len()];
				let [A,B,C] = [a,b,c].map(|i| polygon[i]);
				if (A[0]==B[0]&&B[0]==C[0])||(A[1]==B[1]&&B[1]==C[1]) { println!("{a} {b} {c} -{b} {polygon:?}");  polygon.remove(b); break; }
			}
			for [A, B] in polygon.array_windows() { assert!(A!=B, "{polygon:?}"); }
			for [A, B, C] in polygon.array_windows() { assert!((!(A[0]==B[0]&&B[0]==C[0])||(A[1]==B[1]&&B[1]==C[1])), "{polygon:?}"); }
		} else { i+=1; }
	}
	match polygon[..] {
		/*[A,B,C] => {
			assert!((A[0]==B[0]&&B[0]==C[0])||(A[1]==B[1]&&B[1]==C[1]));
			area
		}*/
		[A,B,C,D] => {
			area + if A[0] == B[0] {
				assert_eq!(B[1], C[1]);
				assert!(B[0] != C[0]);
				assert_eq!(C[0], D[0], "{A:?} {B:?} {C:?} {D:?} {area}");
				assert!(C[1] != D[1]);
				assert_eq!(D[1], A[1]);
				assert!(D[0] != A[0]);
				assert_eq!(C[0]-B[0], D[0]-A[0]);
				assert_eq!(B[1]-A[1], C[1]-D[1]);
				(B[1]-A[1]).abs() as usize * (C[0]-B[0]).abs() as usize
			} else if A[1] == B[1] {
				assert_eq!(B[0], C[0]);
				assert!(B[1] != C[1]);
				assert_eq!(C[1], D[1]);
				assert!(C[0] != D[0]);
				assert_eq!(D[0], A[0]);
				assert!(D[1] != A[1]);
				assert_eq!(C[1]-B[1], D[1]-A[1]);
				assert_eq!(B[0]-A[0], C[0]-D[0]);
				(B[0]-A[0]).abs() as usize * (C[1]-B[1]).abs() as usize
			} else { unreachable!() }
		}
		_ => unreachable!()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_part_one() {
		let result = part_one(&advent_of_code::template::read_file("examples", DAY));
		assert_eq!(result, Some(62));
	}

	#[test]
	fn test_part_two() {
		let result = part_two(&advent_of_code::template::read_file("examples", DAY));
		assert_eq!(result, Some(952408144115));
	}
}
