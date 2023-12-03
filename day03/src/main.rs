use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::cmp::{max, min};


fn main() {
    if let Ok(lines) = read_lines("day_input.txt") {
        let lines: Vec<String> = lines.map(|l| l.unwrap()).collect();
        println!("Part 1: {}", day3_part1(lines.clone()));
        println!("Part 2: {}", day3_part2(lines.clone()));
    }
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn get_current_element(full_line: &str, _i: usize, j: usize) -> char {
    full_line.chars().nth(j).unwrap()
}


fn left_helper(full_line: &str, i: usize, j: usize) -> (i32, i32) {

    let mut left_most_loc = (i as i32, j as i32);
    let safe_j = max(0, j as i32 - 1);

    if full_line.chars().nth(safe_j as usize).unwrap().is_ascii_digit() {

        let mut last_k = j;
        for k in (0..j).rev() {
            let new_digit = full_line.chars().nth(k).unwrap();
            if !new_digit.is_ascii_digit() {
                left_most_loc = (i as i32, last_k as i32);
                break;
            }

            if (k == 0) & (new_digit.is_ascii_digit()) {
                left_most_loc = (i as i32, k as i32);
            }

            last_k = k;
        }
    }
    left_most_loc
}


fn get_full_number_from_leftmost(full_line: &str, j: usize) -> i32 {
    let mut full_number = String::new();
    let mut k = j;
    while (k < full_line.len()) && full_line.chars().nth(k).unwrap().is_ascii_digit() {
        full_number.push(full_line.chars().nth(k).unwrap());
        k += 1;
    }
    full_number.parse::<i32>().unwrap()
}


fn day3_part1(lines: Vec<String>) -> i32 {
    let rows = lines.len();
    let cols = lines[0].len();

    let mut left_most_loc = HashSet::new();

    for i in 0..rows {
        for j in 0..cols {
            let current_element = get_current_element(&lines[i], i, j);
            if !current_element.is_ascii_digit() && current_element != '.' {
                // check left until non digit is found
                let safe_j = max(0, j - 1);
                let left_element = get_current_element(&lines[i], i, safe_j);
                if left_element.is_ascii_digit() {
                    left_most_loc.insert(left_helper(&lines[i], i, j));
                }

                // check right until non digit is found
                let safe_j = min(cols - 1, j + 1);
                let right_element = get_current_element(&lines[i], i, safe_j);
                if right_element.is_ascii_digit() {
                    left_most_loc.insert((i as i32, safe_j as i32));
                }

                // check down until non digit is found
                let safe_i = min(rows - 1, i + 1);
                let safe_j_min = max(0, j - 1);
                let safe_j_max = min(cols - 1, j + 1);

                for k in safe_j_min..=safe_j_max {
                    let lower_element = get_current_element(&lines[safe_i], safe_i, k);
                    if lower_element.is_ascii_digit() {
                        left_most_loc.insert(left_helper(&lines[safe_i], safe_i, k));
                    }
                }

                // check up until non digit is found
                let safe_i = max(0, i - 1);
                for k in safe_j_min..=safe_j_max {
                    let upper_element = get_current_element(&lines[safe_i], safe_i, k);
                    if upper_element.is_ascii_digit() {
                        left_most_loc.insert(left_helper(&lines[safe_i], safe_i, k));
                    }
                }
            }
        }
    }
    let mut summed = 0;
    for (i, j) in left_most_loc {
        summed += get_full_number_from_leftmost(&lines[i as usize], j as usize);
    }
    summed
}


fn day3_part2(lines: Vec<String>) -> i32 {
    let rows = lines.len();
    let cols = lines[0].len();

    let mut gear_map: HashMap<(i32, i32), HashSet<(i32, i32)>> = HashMap::new();

    for i in 0..rows {
        for j in 0..cols {
            let current_element = get_current_element(&lines[i], i, j);
            let mut left_most_loc = HashSet::new();

            if current_element == '*' {
                // check left until non digit is found
                let safe_j = max(0, j - 1);
                let left_element = get_current_element(&lines[i], i, safe_j);
                if left_element.is_ascii_digit() {
                    left_most_loc.insert(left_helper(&lines[i], i, j));
                }

                // check right until non digit is found
                let safe_j = min(cols - 1, j + 1);
                let right_element = get_current_element(&lines[i], i, safe_j);
                if right_element.is_ascii_digit() {
                    left_most_loc.insert((i as i32, safe_j as i32));
                }

                // check down until non digit is found
                let safe_i = min(rows - 1, i + 1);
                let safe_j_min = max(0, j - 1);
                let safe_j_max = min(cols - 1, j + 1);
                for k in safe_j_min..=safe_j_max {
                    let lower_element = get_current_element(&lines[safe_i], safe_i, k);
                    if lower_element.is_ascii_digit() {
                        left_most_loc.insert(left_helper(&lines[safe_i], safe_i, k));
                    }
                }

                // check up until non digit is found
                let safe_i = max(0, i - 1);
                for k in safe_j_min..=safe_j_max {
                    let upper_element = get_current_element(&lines[safe_i], safe_i, k);
                    if upper_element.is_ascii_digit() {
                        left_most_loc.insert(left_helper(&lines[safe_i], safe_i, k));
                    }
                }
                gear_map.insert((i as i32, j as i32), left_most_loc);
            }
        }
    }
    let mut summed = 0;
    for ((_i, _j), left_most_loc) in gear_map {

        // given in prompt that we'll only include when there's 2
        if left_most_loc.len() == 2 {

            // get the two numbers and multiply
            let lhs = left_most_loc.iter().next().unwrap();
            let rhs = left_most_loc.iter().last().unwrap();

            let lhs_sum = get_full_number_from_leftmost(&lines[lhs.0 as usize], lhs.1 as usize);
            let rhs_sum = get_full_number_from_leftmost(&lines[rhs.0 as usize], rhs.1 as usize);

            summed += lhs_sum * rhs_sum;
        }
    }
    summed
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day3_part1() {
        let test_input = vec![
            "467..114..".to_string(),
            "...*......".to_string(),
            "..35..633.".to_string(),
            "......#...".to_string(),
            "617*......".to_string(),
            ".....+.58.".to_string(),
            "..592.....".to_string(),
            "......755.".to_string(),
            "...$.*....".to_string(),
            ".664.598..".to_string(),
        ];
        assert_eq!(day3_part1(test_input), 43_61);
    }

    #[test]
    fn test_day3_part2() {
        let test_input = vec![
            "467..114..".to_string(),
            "...*......".to_string(),
            "..35..633.".to_string(),
            "......#...".to_string(),
            "617*......".to_string(),
            ".....+.58.".to_string(),
            "..592.....".to_string(),
            "......755.".to_string(),
            "...$.*....".to_string(),
            ".664.598..".to_string(),
        ];
        assert_eq!(day3_part2(test_input), 467_835);
    }
}
