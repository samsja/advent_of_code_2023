use std::env;
use std::fs::File;
use std::io::prelude::*;

fn check_game(_input: &str) -> bool {
    // input example "1 green, 4 blue;"
    true
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
    // input example "Game 1: 1 green, 4 blue;"

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
