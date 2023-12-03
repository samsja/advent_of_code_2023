use std::env;
use std::fs::File;
use std::io::prelude::*;

fn read_file(path: String) -> std::io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn get_first_digit<I: Iterator<Item = char>>(chars: I) -> Result<usize, &'static str> {
    for c in chars {
        if c.is_ascii_digit() {
            return Ok(c.to_digit(10).unwrap() as usize);
        }
    }

    return Err("No digit found");
}

fn get_calib_line_value(input: &str) -> usize {
    let left_digit = get_first_digit(input.chars()).unwrap();
    let right_digit = get_first_digit(input.chars().rev().into_iter()).unwrap();

    return left_digit * 10 + right_digit;
}
fn calib_sum_value(_inputs: Vec<&str>) -> usize {
    return _inputs.into_iter().map(get_calib_line_value).sum();
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
    let sum = calib_sum_value(contents);
    println!("Calibration sum is: {}", sum);
    Ok(())
}
