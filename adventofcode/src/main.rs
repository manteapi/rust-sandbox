use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    day1();
    day2();
}

fn day1() {
    let result = parse_day1_input("day/1/input.txt");
    match result {
        Ok(input) => {
            println!(
                "{:?}",
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

fn day2() {
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
            println!("Result: {:?}", forward_number * (-up_number + down_number))
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
