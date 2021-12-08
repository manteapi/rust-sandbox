use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    day1_part_1();
    day1_part_2();
    day2_part1();
}

fn day1_part_1() {
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

fn day1_part_2() {
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

            println!("Down: {:?}", down_number);
            println!("Up: {:?}", up_number);
            println!("Depth: {:?}", up_number - down_number);
            println!("Forward: {:?}", forward_number);
            println!(
                "Day 2 - Part 1: {:?}",
                forward_number * (-up_number + down_number)
            )
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
