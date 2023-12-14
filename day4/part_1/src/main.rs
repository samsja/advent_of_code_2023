use std::env;
use std::fs::File;
use std::io::prelude::*;

fn get_num(line: &str) -> Vec<usize> {
    // 41 48 83 86 17
    let mut num = Vec::<usize>::new();
    let mut current_num = String::new();

    for c in line.chars().chain(" ".chars()) {
        if c.is_ascii_digit() {
            current_num.push(c);
        } else {
            if !current_num.is_empty() {
                num.push(current_num.parse::<usize>().unwrap());
                current_num.clear();
            }
        }
    }

    num
}

fn game_point(line: &str) -> usize {
    // input like 'Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53'

    let game_line = line.split(":").collect::<Vec<_>>()[1];

    let two_part: Vec<&str> = game_line.split("|").collect();

    let left_num = get_num(two_part[0]);
    let right_num = get_num(two_part[1]);

    let mut count: u32 = 0;

    for num in right_num.iter() {
        if left_num.contains(num) {
            count += 1;
        }
    }
    if count == 0 {
        return 0;
    }
    let two: usize = 2;
    two.pow(count - 1)
}

fn total_points(input: Vec<&str>) -> usize {
    input.into_iter().map(game_point).sum()
}

fn read_file(path: String) -> std::io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cargo run <file_name>");
        std::process::exit(1);
    }

    let file_name = &args[1];
    let bindings = read_file(file_name.to_string())?;
    let contents: Vec<&str> = bindings.lines().collect();
    let sum = total_points(contents);
    println!("Calibration sum is: {}", sum);
    Ok(())
}
