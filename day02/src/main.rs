use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {
    if let Ok(lines) = read_lines("input.txt") {
        let lines: Vec<String> = lines.map(|l| l.unwrap()).collect();
        println!("Part 1: {}", day2_part1(lines.clone()));
        println!("Part 2: {}", day2_part2(lines.clone()));
    }
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn helper_determine_if_safe_part1(games: Vec<&str>, color_counts: HashMap<String, i32>) -> bool {
    // games example is ["3 blue, 4 red", "1 red, 2 green, 6 blue", "2 green"]
    for game in games {
        // game example is "3 blue, 4 red"
        let colors = game.split(',').collect::<Vec<&str>>();
        for color in colors {
            // color example is "3 blue"
            let color = color.trim();
            
            // color_num example is 3 and color_type example is "blue"
            let color_num = color.split(' ').next().unwrap().parse::<i32>().unwrap();
            let color_type = color.split(' ').last().unwrap();

        // if the color_num is greater than the color_count, return false (not feasible)
        if color_num > color_counts[color_type] {
                return false;
            }
        }
    }
    // if no issues, return true (feasible)
    true
}


fn day2_part1(lines: Vec<String>) -> i32 {
    let mut summed = 0;

    // initialize the color counts
    let mut color_counts = HashMap::new();

    color_counts.insert("red".to_string(), 12);
    color_counts.insert("green".to_string(), 13);
    color_counts.insert("blue".to_string(), 14);

    for line in lines {
        // line example is "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
        let game_number = line.split(':').next().unwrap().split(' ').last().unwrap().parse::<i32>().unwrap();
        let games = line.split(':').last().unwrap().split(';').collect::<Vec<&str>>();

        // if the game is feasible, add the game number to the sum
        if helper_determine_if_safe_part1(games, color_counts.clone()) {
            summed += game_number;
        }
    }
    summed
}


fn helper_get_power_of_maxes_part2(games: Vec<&str>) -> i32 {
    // initialize maxes as 1 (safe multiplier if color doesn't exist in game)
    let mut red_max = 1;
    let mut green_max = 1;
    let mut blue_max = 1;

    for game in games {
        //game example is "3 blue, 4 red"
        let colors = game.split(',').collect::<Vec<&str>>();

        for color in colors {
            // color example is "3 blue"
            let color = color.trim();
            
            // color_num example is 3 and color_type example is "blue"
            let color_num = color.split(' ').next().unwrap().parse::<i32>().unwrap();
            let color_type = color.split(' ').last().unwrap();

            match color_type {
                "red" => {
                    if color_num > red_max {
                        red_max = color_num;
                    }
                },
                "green" => {
                    if color_num > green_max {
                        green_max = color_num;
                    }
                },
                "blue" => {
                    if color_num > blue_max {
                        blue_max = color_num;
                    }
                },
                _ => (),
            }
        }
    }
    // return the product of the maxes
    red_max * green_max * blue_max
}


fn day2_part2(lines: Vec<String>) -> i32 {
    let mut summed = 0;

    for line in lines {
        let games = line.split(':').last().unwrap().split(';').collect::<Vec<&str>>();
        summed += helper_get_power_of_maxes_part2(games);
    }
    summed
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day2_part1() {
        let test_input = vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string(),
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_string(),
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".to_string(),
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red".to_string(),
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string(),
        ];
        assert_eq!(day2_part1(test_input), 8);
    }

    #[test]
    fn test_day2_part2() {
        let test_input = vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string(),
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_string(),
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".to_string(),
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red".to_string(),
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string(),
        ];
        assert_eq!(day2_part2(test_input), 2286);
    }
}
