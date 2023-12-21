advent_of_code::solution!(20);

#[fehler::throws(as Option)] pub fn part_one(input: &str) -> usize {
	let mut modules = input.lines().map(|line| {
		let (prefix_name, outputs) = line.split_once(" -> ").unwrap();
		let (prefix, name) = ["%","&",""].iter().find_map(|&prefix| prefix_name.strip_prefix(prefix).map(|name| (prefix, name))).unwrap();
		let outputs = outputs.split(", ").collect::<Box<[_]>>();
		(name, prefix, outputs)
	}).collect::<Vec<_>>();
	modules.push(("output", "", Box::new([])));
	let mut flipflops = modules.iter().filter_map(|&(flipflop, prefix, _)| (prefix=="%").then_some((flipflop, false))).collect::<Box<_>>();
	let mut conjunctions = modules.iter().filter_map(|&(conjunction, prefix, _)| (prefix=="&").then(||
		(conjunction, modules.iter().filter_map(|(input, _, outputs)| outputs.contains(&conjunction).then_some((*input,false))).collect::<Box<_>>())
	)).collect::<Box<_>>();
	let mut pulses = [0, 0];
	fn trigger(modules: &[(&str, &str, Box<[&str]>)], flipflops: &mut [(&str, bool)], conjunctions: &mut [(&str, Box<[(&str, bool)]>)], pulses: &mut [usize; 2], module: &str, high: bool, from: &str) {
		println!("{from} -{}> {module}", if high {"high"} else {"low"});
		pulses[if high {1} else {0}] += 1;
		let (_, prefix, outputs) = modules.iter().find(|&&(name,_,_)| name==module).expect(module);
		match *prefix {
			"" => for output in &**outputs { trigger(modules, flipflops, conjunctions, pulses, output, high, module); }
			"%" => if !high {
				let (_, state) = flipflops.iter_mut().find(|&&mut(flipflop,_)| flipflop==module).unwrap();
				match *state {
					false => {
						*state = true;
						for output in &**outputs { trigger(modules, flipflops, conjunctions, pulses, output, true, module); }
					},
					true => {
						*state = false;
						for output in &**outputs { trigger(modules, flipflops, conjunctions, pulses, output, false, module); }
					}
				}
			}
			"&" => {
				let (_, inputs) = conjunctions.iter_mut().find(|&&mut(conjunction,_)| conjunction==module).unwrap();
				inputs.iter_mut().find(|&&mut (input,_)| input==from).expect(from).1 = high;
				let pulse = if inputs.iter().all(|&(_, high)| high) { false } else { true };
				for output in &**outputs { trigger(modules, flipflops, conjunctions, pulses, output, pulse, module); }
			}
			_ => unreachable!()
		}
	}
	for _ in 0..4 { trigger(&modules, &mut flipflops, &mut conjunctions, &mut pulses, "broadcaster", false, "button"); println!(); }
	println!("{pulses:?}");
	pulses[0]*pulses[1]
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
		assert_eq!(result, Some(11687500));
	}

	#[test]
	fn test_part_two() {
		let result = part_two(&advent_of_code::template::read_file("examples", DAY));
		assert_eq!(result, None);
	}
}
