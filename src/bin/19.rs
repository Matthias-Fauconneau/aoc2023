advent_of_code::solution!(19);

#[fehler::throws(as Option)] pub fn part_one(input: &str) -> u32 {
	let (workflows, parts) = input.split_once("\n\n").unwrap();
	let workflows = workflows.lines().map(|line| {
		let (name, line) = line.split_once('{').unwrap();
		let line = line.strip_suffix('}').unwrap();
		let rules = line.split(',').map(|rule| {
			let (condition, target) = rule.split_once(':').unwrap_or(("",rule));
			let condition = (!condition.is_empty()).then(|| {
				let (category, condition) = condition.split_at(1);
				let (condition, threshold) = condition.split_at(1);
				("xmas".find(category).unwrap(), condition, threshold.parse::<u32>().unwrap())
			});
			(condition, target)
		}).collect::<Box<[_]>>();
		(name, rules)
	}).collect::<Box<[_]>>();
	parts.lines().filter_map(|part| {
		let part : [_; 4] = part.strip_prefix('{').unwrap().strip_suffix('}').unwrap().split(',').enumerate().map(|(i,rating)| rating.strip_prefix(["x=","m=","a=","s="][i]).unwrap().parse::<u32>().unwrap()).collect::<Vec<_>>().try_into().unwrap();
		let mut workflow = "in";
		while !["A","R"].contains(&workflow) {
			workflow = workflows.iter().find(|w| w.0 == workflow).unwrap().1.iter().find(|&&(condition, _)| {
				let Some((category, condition, threshold)) = condition else { return true; };
				let rating = part[category];
				match condition {
					"<" => rating < threshold,
					">" => rating > threshold,
					_ => unreachable!()
				}
			}).unwrap().1;
		}
		(workflow == "A").then_some(part.iter().sum::<u32>())
	}).sum()
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
		assert_eq!(result, Some(19114));
	}

	/*#[test]
	fn test_part_two() {
		let result = part_two(&advent_of_code::template::read_file("examples", DAY));
		assert_eq!(result, Some(167409079868000));
	}*/
}
