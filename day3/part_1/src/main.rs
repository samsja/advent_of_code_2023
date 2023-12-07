use std::env;
use std::fs::File;
use std::io::prelude::*;

fn look_for_number(line: &str, i: usize) -> usize {
    0
}

fn is_special(c: char) -> bool {
    return !(c.is_ascii_digit() || c == '.');
}

fn val_trio(top: &str, mid: &str, bottom: &str) -> usize {
    let mut val: usize = 0;

    let mid_iter = ".".chars().chain(mid.chars()).chain(".".chars());
    // add . before and after
    for (i, c) in mid_iter.enumerate() {
        if is_special(c) {
            val += look_for_number(top, i - 1)
                + look_for_number(bottom, i - 1)
                + look_for_number(mid, i - 1)
        }
    }

    val
}

fn game_sum(input: Vec<&str>) -> usize {
    let mut sum: usize = 0;

    for i in 1..input.len() - 1 {
        sum += val_trio(input[i - 1], input[i], input[i + 1])
    }

    sum
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
