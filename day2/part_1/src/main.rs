use core::panic;
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn check_part(part: &str) -> Result<bool, String> {
    // input example "1 green, 4 blue"
    for sub_part in part.split(",") {
        // sub_part: " 1 green"
        let sub_part = &sub_part[1..];
        // sub_part: "1 green"

        let splits: Vec<_> = sub_part.split(" ").collect();

        let num = splits[0].parse::<usize>().unwrap();
        let color = splits[1];

        let valid = match color {
            "red" => num <= 12,
            "green" => num <= 13,
            "blue" => num <= 14,
            _ => return Err(format!("color is not valid {color}")),
        };

        if !valid {
            return Ok(false);
        }
    }

    return Ok(true);
}

fn check_game(input: &str) -> bool {
    // input example " 1 green, 4 blue; 1 blue, 2 green, 1 red;"
    for part in input.split(";") {
        if !check_part(part).unwrap() {
            return false;
        }
    }

    return true;
}

fn get_game_id(input: &str) -> Result<usize, String> {
    // input example "Game 1"
    let mut last_i = 0;

    for (i, c) in input.chars().rev().enumerate() {
        if !c.is_ascii_digit() {
            last_i = i;
            break;
        }
    }

    if (last_i == input.len() - 1) || (last_i == 0) {
        let error_msg = format!("cannot get game id from {input}");
        return Err(error_msg);
    } else {
        Ok(input[input.len() - last_i..].parse::<usize>().unwrap())
    }
}

fn game_value<'a>(input: &'a str) -> usize {
    // input example "Game 1: 1 green, 4 blue; 1 blue, 2 green, 1 red;"

    for (i, c) in input.chars().enumerate() {
        if c == ':' {
            if check_game(&input[i + 1..]) {
                return get_game_id(&input[..i]).unwrap();
            } else {
                return 0;
            }
        }
    }

    panic!("cannot split by id on {input}");
}

fn game_sum(input: Vec<&str>) -> usize {
    return input.into_iter().map(game_value).sum();
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
    let sum = game_sum(contents);
    println!("Calibration sum is: {}", sum);
    Ok(())
}
