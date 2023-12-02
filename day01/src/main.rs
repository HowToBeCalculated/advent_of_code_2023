use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const NUMBER_NAMES: [&str; 10] = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];


fn main() {
    if let Ok(lines) = read_lines("day_input.txt") {
        let lines: Vec<String> = lines.map(|l| l.unwrap()).collect();
        println!("Part 1: {}", day1_part1(lines.clone()));
        println!("Part 2: {}", day1_part2(lines));
    }
    
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn helper_get_first_number_part1(input: &str) -> char {
    // initialize first_number to '0' in case no number is found
    let mut first_number = '0';

    // iterate over the number names and find the first one in the input
    for c in input.chars() {
        if c.is_digit(10) {
            first_number = c;
            break;
        }
    }
    first_number
}


fn day1_part1(lines: Vec<String>) -> i32 {
    let mut summed = 0;

    for line in lines {
        // get the first number in the line
        let first_number = helper_get_first_number_part1(&line);

        // reverse the line and get the first number in the reversed line
        let reversed_line = line.chars().rev().collect::<String>();
        let second_number = helper_get_first_number_part1(&reversed_line);

        // concatenate the two numbers and add to the sum
        let concat_number = format!("{}{}", first_number, second_number);
        summed += concat_number.parse::<i32>().unwrap();
    }
    summed
}


fn helper_get_first_number_part2(input: &str, reversed: bool) -> char {
    // initialize first_number to '0' in case no number is found
    let mut first_number = '0';

    // reverse the number names if we are looking for the second number in reversed line
    let nums: Vec<String> = if reversed {
        NUMBER_NAMES.iter().map(|s| s.chars().rev().collect()).collect()
    } else {
        NUMBER_NAMES.iter().map(|s| s.to_string()).collect()
    };    

    // initialize min_idx to the length of the input string
    let mut min_idx = input.len();

    // iterate over the number names and find the first one in the input
    for (index, num) in nums.iter().enumerate() {
        if input.contains(num) {
            let idx = input.find(&num.to_string()).unwrap_or(input.len());
            if idx < min_idx {
                first_number = index.to_string().chars().next().unwrap();
                min_idx = idx;
            }
        }
    }

    // iterate over the characters before the first number and find the first digit
    for c in input.chars().take(min_idx) {
        if c.is_digit(10) {
            first_number = c;
            break;
        }
    }
    first_number
}


fn day1_part2(lines: Vec<String>) -> i32 {
    let mut summed = 0;

    for line in lines {
        // get the first number in the line
        let first_number = helper_get_first_number_part2(&line, false);

        // reverse the line and get the first number in the reversed line
        let reversed_line = line.chars().rev().collect::<String>();
        let second_number = helper_get_first_number_part2(&reversed_line, true);

        // concatenate the two numbers and add to the sum
        let concat_number = format!("{}{}", first_number, second_number);
        summed += concat_number.parse::<i32>().unwrap();
    }
    summed
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day1_part1() {
        let test_input = vec![
            "1abc2".to_string(),
            "pqr3stu8vwx".to_string(),
            "a1b2c3d4e5f".to_string(),
            "treb7uchet".to_string()
        ];
        assert_eq!(day1_part1(test_input), 142);
    }

    #[test]
    fn test_day1_part2() {
        let test_input = vec![
            "two1nine".to_string(),
            "eightwothree".to_string(),
            "abcone2threexyz".to_string(),
            "xtwone3four".to_string(),
            "4nineeightseven2".to_string(),
            "zoneight234".to_string(),
            "7pqrstsixteen".to_string()
        ];
        assert_eq!(day1_part2(test_input), 281);
    }
}
