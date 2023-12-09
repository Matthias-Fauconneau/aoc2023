advent_of_code::solution!(2);

#[fehler::throws(as Option)] pub fn part_one(input: &str) -> u32 {
    let components = ["red", "green", "blue"];
    let bag = [12,13,14];
    input.lines().filter_map(|mut line| {
        line = line.strip_prefix("Game ").unwrap();
        let number = |line: &mut &str| -> u32 { let (number, suffix) = line.split_once(|c| !char::is_digit(c, 10)).unwrap(); *line=suffix; number.parse().unwrap() };
        let id = number(&mut line);
        for reveal in line.split(';') {
            for component in reveal.split(',') {
                let (count, component) = component.trim().split_once(' ').unwrap();
                let count : u32 = count.parse().unwrap();
                if count > bag[components.iter().position(|&c| c == component).unwrap()] { return None; }
            }
        }
        Some(id)
    }).sum()
}

#[fehler::throws(as Option)]  pub fn part_two(input: &str) -> u32 {
    let components = ["red", "green", "blue"];
    input.lines().filter_map(|mut line| {
        line = line.strip_prefix("Game ").unwrap();
        let number = |line: &mut &str| -> u32 { let (number, suffix) = line.split_once(|c| !char::is_digit(c, 10)).unwrap(); *line=suffix; number.parse().unwrap() };
        let _id = number(&mut line);
        let mut bag = [0,0,0];
        for reveal in line.split(';') {
            for component in reveal.split(',') {
                let (count, component) = component.trim().split_once(' ').unwrap();
                let count : u32 = count.parse().unwrap();
                let bag = &mut bag[components.iter().position(|&c| c == component).unwrap()];
                *bag = (*bag).max(count);
            }
        }
        Some(bag.iter().product::<u32>())
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
