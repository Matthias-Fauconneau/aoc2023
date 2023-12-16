//#![feature(round_char_boundary)]
advent_of_code::solution!(1);

#[fehler::throws(as Option)] pub fn part_one(input: &str) -> u32 {
    input.lines().map(|line| {
        let mut digits = line.chars().filter_map(|c| c.to_digit(10));
        let mut number = digits.next().unwrap();
        if  let Some(digit) =  digits.last() { number = number * 10 + digit; }
        else { number = number * 10 + number; } // A digit alone on a line get used twice :(
        dbg!(line, number);
        number
    }).sum()
}

#[fehler::throws(as Option)] pub fn part_two(input: &str) -> u32 {
    let digits = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    input.lines().map(|mut line| {
        print!("{line} ");
        let mut digits = std::iter::from_fn(||{ while !line.is_empty() {
            if let Some(digit) = line.chars().next().unwrap().to_digit(10) {
                line = line.strip_prefix(char::from_digit(digit, 10).unwrap()).unwrap();
                return Some(digit)
            }
            else {
                fn split_first(line:&str) -> &str { &line[/*line.ceil_char_boundary*/(1)..] } // Splits after first character
                for (digit, token) in digits.iter().enumerate() {
                    if let Some(after_prefix) = line.strip_prefix(token) {
                        if false { // Robotic sequential reading: eighthree: eight hree, sevenine: seven ine
                            line = after_prefix;
                        } else { // /!\ Elfish poets: eighthree: eight three, sevenine: seven ine
                            line = split_first(line);
                        }
                        return Some(digit as _);
                    }
                }
                line = split_first(line);
            }
        } None });
        let mut number = digits.next().unwrap();
        if  let Some(digit) =  digits.last() { number = number * 10 + digit; }
        else { number = number * 10 + number; }
        println!("{number}");
        number
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(281));
    }
}
