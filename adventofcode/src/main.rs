use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    day1_part1();
    day1_part2();
    day2_part1();
    day2_part2();
    day3_part1();
    day3_part2();
}

fn day1_part1() {
    let result = parse_day1_input("day/1/input.txt");
    match result {
        Ok(input) => {
            println!(
                "Day 1 - Part 1 : {:?}",
                compute_orientation(input)
                    .iter()
                    .filter(|&&orientation| return orientation == true)
                    .collect::<Vec<&bool>>()
                    .len()
            );
        }
        Err(error) => println!("{:?}", error),
    }
}

fn compute_3_size_window(input: Vec<i32>) -> Vec<i32> {
    let mut output: Vec<i32> = Vec::new();
    for (index, _) in input.iter().enumerate() {
        if index >= 2 {
            let window_value: i32 = input[index - 2] + input[index - 1] + input[index];
            output.push(window_value);
        }
    }
    return output;
}

fn day1_part2() {
    let result = parse_day1_input("day/1/input.txt");
    match result {
        Ok(input) => {
            let windows: Vec<i32> = compute_3_size_window(input);
            println!(
                "Day 1 - Part 2 : {:?}",
                compute_orientation(windows)
                    .iter()
                    .filter(|&&orientation| return orientation == true)
                    .collect::<Vec<&bool>>()
                    .len()
            );
        }
        Err(error) => println!("{:?}", error),
    }
}

fn day2_part1() {
    let result = parse_day2_input("day/2/input.txt");
    match result {
        Ok(input) => {
            let mut forward_number: i32 = 0;
            let mut down_number: i32 = 0;
            let mut up_number: i32 = 0;

            for data in input {
                match data[1].parse::<i32>() {
                    Ok(number_of_steps) => {
                        if data[0] == "down" {
                            down_number += number_of_steps;
                        } else if data[0] == "up" {
                            up_number += number_of_steps;
                        } else if data[0] == "forward" {
                            forward_number += number_of_steps;
                        } else {
                            println!("Some error occured");
                        }
                    }
                    Err(error) => {
                        println!("{:?}", error);
                    }
                }
            }

            let depth_number = down_number - up_number;
            println!("Day 2 - Part 1: {:?}", forward_number * (depth_number))
        }
        Err(error) => {
            println!("{:?}", error);
        }
    }
}

fn day2_part2() {
    let result = parse_day2_input("day/2/input.txt");
    match result {
        Ok(input) => {
            let mut aim: i32 = 0;
            let mut horizontal: i32 = 0;
            let mut depth: i32 = 0;

            for data in input {
                match data[1].parse::<i32>() {
                    Ok(number_of_steps) => {
                        if data[0] == "down" {
                            aim += number_of_steps;
                        } else if data[0] == "up" {
                            aim -= number_of_steps;
                        } else if data[0] == "forward" {
                            horizontal += number_of_steps;
                            depth += aim * number_of_steps;
                        } else {
                            println!("Some error occured");
                        }
                    }
                    Err(error) => {
                        println!("{:?}", error);
                    }
                }
            }

            println!("Day 2 - Part 2: {:?}", depth * horizontal);
        }
        Err(error) => {
            println!("{:?}", error);
        }
    }
}

fn find_most_common_bit(input: &Vec<String>) -> String {
    let number_of_one: usize = input.iter().filter(|value| value.as_str() == "1").count();
    let number_of_zero: usize = input.iter().filter(|value| value.as_str() == "0").count();
    if number_of_zero > number_of_one {
        return "0".to_string();
    } else {
        return "1".to_string();
    }
}

fn find_less_common_bit(input: &Vec<String>) -> String {
    let number_of_one: usize = input.iter().filter(|value| value.as_str() == "1").count();
    let number_of_zero: usize = input.iter().filter(|value| value.as_str() == "0").count();
    if number_of_zero <= number_of_one {
        return "0".to_string();
    } else {
        return "1".to_string();
    }
}

fn parse_bit(input: &Vec<String>, index: usize) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();
    for line in input {
        output.push(line.chars().nth(index).unwrap().to_string());
    }
    return output;
}

fn day3_part1() {
    let result = parse_day3_input("day/3/input.txt");
    match result {
        Ok(input) => {
            let mut gamma_rate_as_string: String = String::new();
            let mut epsilon_rate_as_string: String = String::new();
            for i in 0..12 {
                epsilon_rate_as_string
                    .push_str(find_less_common_bit(&parse_bit(&input, i)).as_str());
                gamma_rate_as_string.push_str(find_most_common_bit(&parse_bit(&input, i)).as_str());
            }
            let gamma_rate: isize =
                isize::from_str_radix(gamma_rate_as_string.as_str(), 2).unwrap();
            let epsilon_rate: isize =
                isize::from_str_radix(epsilon_rate_as_string.as_str(), 2).unwrap();
            let power_consumption: isize = gamma_rate * epsilon_rate;
            println!("Day 3 - Part 1: {:?}", power_consumption);
        }
        Err(error) => {
            println!("{:?}", error);
        }
    }
}

fn filter_binary(input: &Vec<String>, index: usize, value: String) -> Vec<String> {
    return input
        .iter()
        .filter(|number| number.chars().nth(index).unwrap().to_string() == value)
        .cloned()
        .collect();
}

fn drain_most_common_bit(input: &Vec<String>) -> String {
    let mut index: usize = 0;
    let mut filtered_input: Vec<String> = input.to_vec();
    while filtered_input.len() > 1 {
        let most_common_bit: String = find_most_common_bit(&parse_bit(&filtered_input, index));
        filtered_input = filter_binary(&filtered_input, index, most_common_bit);
        index = index + 1;
    }
    return filtered_input[0].to_string();
}

fn drain_least_common_bit(input: &Vec<String>) -> String {
    let mut index: usize = 0;
    let mut filtered_input: Vec<String> = input.to_vec();
    while filtered_input.len() > 1 {
        let least_common_bit: String = find_less_common_bit(&parse_bit(&filtered_input, index));
        filtered_input = filter_binary(&filtered_input, index, least_common_bit);
        index = index + 1;
    }
    return filtered_input[0].to_string();
}

fn day3_part2() {
    let result = parse_day3_input("day/3/input.txt");
    match result {
        Ok(input) => {
            let oxgen_generator_rating: isize =
                isize::from_str_radix(&drain_most_common_bit(&input), 2).unwrap();
            let co2_scrubbe_rating: isize =
                isize::from_str_radix(&drain_least_common_bit(&input), 2).unwrap();
            println!(
                "Day 3 - Part 2: {:?}",
                oxgen_generator_rating * co2_scrubbe_rating
            );
        }
        Err(error) => {
            println!("{:?}", error);
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn compute_orientation(input: Vec<i32>) -> Vec<bool> {
    let mut output: Vec<bool> = Vec::new();
    for (index, number) in input.iter().enumerate() {
        if index > 0 {
            let previous_index: usize = index - 1;
            let previous_number: i32 = input[previous_index];
            let orientation: bool = (number > &previous_number) as bool;
            output.push(orientation);
        }
    }
    return output;
}

fn parse_day3_input<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    if let Ok(lines) = read_lines(filename) {
        return lines.collect();
    } else {
        return Ok(Vec::new());
    }
}

fn parse_day2_input<P>(filename: P) -> io::Result<Vec<Vec<String>>>
where
    P: AsRef<Path>,
{
    if let Ok(lines) = read_lines(filename) {
        let lines_as_strings: io::Result<Vec<String>> = lines.collect();
        match lines_as_strings {
            Ok(result) => {
                return Ok(result
                    .iter()
                    .map(|line| line.split(" ").map(|value| value.to_string()).collect())
                    .collect());
            }
            Err(e) => {
                return Err(e);
            }
        }
    } else {
        return Ok(Vec::new());
    }
}

fn parse_day1_input<P>(filename: P) -> io::Result<Vec<i32>>
where
    P: AsRef<Path>,
{
    if let Ok(lines) = read_lines(filename) {
        let lines_as_strings: io::Result<Vec<String>> = lines.collect();
        match lines_as_strings {
            Ok(result) => {
                let casts: Vec<i32> = result
                    .iter()
                    .map(|value| value.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();
                return Ok(casts);
            }
            Err(e) => {
                return Err(e);
            }
        }
    } else {
        return Ok(Vec::new());
    }
}
