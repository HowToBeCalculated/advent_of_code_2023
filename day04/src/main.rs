use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {
    if let Ok(lines) = read_lines("day_input.txt") {
        let lines: Vec<String> = lines.map(|l| l.unwrap()).collect();
        println!("Part 1: {}", day4_part1(lines.clone()));
        println!("Part 2: {}", day4_part2(lines.clone()));
    }
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn digest_line(line: &str) -> (i32, Vec<i32>, Vec<i32>) {
    let mut split_line = line.split(": ");
    let card_number = split_line.next().unwrap().split_whitespace().nth(1).unwrap().parse::<i32>().unwrap();
    let mut split_line = split_line.next().unwrap().split(" | ");
    let left_side = split_line.next().unwrap().split_whitespace().map(|x| x.trim().parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let right_side = split_line.next().unwrap().split_whitespace().map(|x| x.trim().parse::<i32>().unwrap()).collect::<Vec<i32>>();
    (card_number, left_side, right_side)
}


fn day4_part1(lines: Vec<String>) -> i32 {
    let mut summed = 0;
    let mut card_data = HashMap::new();

    // go by line and get a hashmap of card number to (left_side (winning numbers), right_side (played numbers))
    for line in lines {
        let (card_number, left_side, right_side) = digest_line(&line);
        card_data.insert(card_number, (left_side, right_side));
    }

    // go through each card and check if the right side contains any of the winning numbers
    for (_card_number, (left_side, right_side)) in card_data.iter() {
        let mut count_of_matches = 0;
    
        for winning_number in left_side {
            if right_side.contains(winning_number) {
                count_of_matches += 1;
            }
        }

        // if there are matches, add 2^(count_of_matches - 1) to the total
        if count_of_matches != 0 {
            summed += 2_i32.pow(count_of_matches - 1);
        }
    }
    summed
}

fn day4_part2(lines: Vec<String>) -> i32 {
    let mut card_data = HashMap::new();

    // go by line and get a hashmap of card number to (left_side (winning numbers), right_side (played numbers))
    for line in lines {
        let (card_number, left_side, right_side) = digest_line(&line);
        card_data.insert(card_number, (left_side, right_side));
    }

    // record having 1 original copy of each card
    let num_of_cards = card_data.len();
    let mut card_record : Vec<i32> = vec![1; num_of_cards];

    // go through each card (starts at #1)
    for i in 1..=(num_of_cards) {
        let (card_number, (left_side, right_side)) = card_data.get_key_value(&(i as i32)).unwrap();
        let idx = *card_number as usize - 1;

        // record number of matches
        let mut count_of_matches = 0;
    
        for winning_number in left_side {
            if right_side.contains(winning_number) {
                count_of_matches += 1;
            }
        }

        // record the number of cards (copies plus the original 1)
        let num_of_copies_plus_original = card_record[idx];

        // add the number of copies to the cards # of matches out
        for j in 0..count_of_matches {
            if j as usize + *card_number as usize >= num_of_cards {
                break;
            }
            card_record[*card_number as usize + j as usize] += num_of_copies_plus_original;
        }
    }
    card_record.iter().sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day4_part1() {
        let test_input = vec![
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".to_string(),
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19".to_string(),
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1".to_string(),
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83".to_string(),
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36".to_string(),
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11".to_string(),
        ];
        assert_eq!(day4_part1(test_input), 13);
    }

    #[test]
    fn test_day4_part2() {
        let test_input = vec![
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".to_string(),
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19".to_string(),
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1".to_string(),
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83".to_string(),
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36".to_string(),
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11".to_string(),
        ];
        assert_eq!(day4_part2(test_input), 30);
    }
}
