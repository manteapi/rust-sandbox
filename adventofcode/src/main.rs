use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let result = parse_input("../day/1/input");
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

fn parse_input<P>(filename: P) -> io::Result<Vec<i32>>
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
