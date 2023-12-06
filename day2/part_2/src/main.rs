use core::panic;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn parse_val(part: &str) -> HashMap<&str, usize> {
    let mut map = HashMap::<&str, usize>::new();
    map.insert("blue", 0);
    map.insert("green", 0);
    map.insert("red", 0);

    for sub_part in part.split(",") {
        // sub_part: " 1 green"
        let sub_part = &sub_part[1..];
        // sub_part: "1 green"

        let splits: Vec<_> = sub_part.split(" ").collect();

        let num = splits[0].parse::<usize>().unwrap();
        let color = splits[1];

        map.insert(color, num);
    }

    map
}

fn get_game_power(input: &str) -> usize {
    // input example " 1 green, 4 blue; 1 blue, 2 green, 1 red;"

    let mut max_red: usize = 0;
    let mut max_green: usize = 0;
    let mut max_blue: usize = 0;

    for part in input.split(";") {
        let map_part = parse_val(part);

        let new_blue: usize = map_part["blue"];
        if max_blue < new_blue {
            max_blue = new_blue;
        }

        let new_green: usize = map_part["green"];
        if max_green < new_green {
            max_green = new_green;
        }

        let new_red: usize = map_part["red"];
        if max_red < new_red {
            max_red = new_red;
        }
    }

    return max_blue * max_green * max_red;
}

fn game_value<'a>(input: &'a str) -> usize {
    // input example "Game 1: 1 green, 4 blue; 1 blue, 2 green, 1 red;"

    for (i, c) in input.chars().enumerate() {
        if c == ':' {
            return get_game_power(&input[i + 1..]);
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
