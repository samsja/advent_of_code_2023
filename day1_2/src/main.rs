use std::env;
use std::fs::File;
use std::io::prelude::*;

fn read_file(path: String) -> std::io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn get_n_carac(chars: Vec<char>, i: usize, n: usize, reverse: bool) -> Option<String> {
    let maybe_n_carac = chars.get(i..i + n);

    if maybe_n_carac.is_none() {
        return None;
    } else {
        let n_caracs: String = if !reverse {
            maybe_n_carac.unwrap().iter().collect()
        } else {
            maybe_n_carac.unwrap().iter().rev().collect()
        };
        return Some(n_caracs);
    }
}

fn get_first_digit(input: &str, reverse: bool) -> Result<usize, String> {
    let chars: Vec<char> = if reverse {
        input.chars().rev().collect()
    } else {
        input.chars().collect()
    };

    for i in 0..chars.len() {
        let carac = chars[i];

        if carac.is_ascii_digit() {
            return Ok(carac.to_digit(10).unwrap() as usize);
        }

        let maybe_three_carac = get_n_carac(chars.clone(), i, 3, reverse);

        match maybe_three_carac {
            Some(three_carac) => match three_carac.as_str() {
                "one" => return Ok(1),
                "two" => return Ok(2),
                "six" => return Ok(6),
                _ => (),
            },
            None => (),
        }

        let maybe_four_carac = get_n_carac(chars.clone(), i, 4, reverse);

        match maybe_four_carac {
            Some(four_carac) => match four_carac.as_str() {
                "four" => return Ok(4),
                "five" => return Ok(5),
                "nine" => return Ok(9),
                _ => (),
            },
            None => (),
        }

        let maybe_five_carac = get_n_carac(chars.clone(), i, 5, reverse);

        match maybe_five_carac {
            Some(five_carac) => match five_carac.as_str() {
                "three" => return Ok(3),
                "seven" => return Ok(7),
                "eight" => return Ok(8),
                _ => (),
            },
            None => (),
        }
    }

    let error_msg = format!("Noo digit found in {}", String::from(input));

    return Err(error_msg);
}

fn get_calib_line_value(input: &str) -> usize {
    let left_digit = get_first_digit(input, false).unwrap();
    let right_digit = get_first_digit(input, true).unwrap();

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
