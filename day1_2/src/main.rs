use std::env;
use std::fs::File;
use std::io::prelude::*;

fn read_file(path: String) -> std::io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

//one 3
//two 3
//six 3

//four 4
//five 4
//nine 4

//three 5
//seven 5
//eight 5

fn get_first_digit(input: &str, left: bool) -> Result<usize, String> {
    let chars: Vec<char> = input.chars().collect();

    for i in 0..chars.len() {
        let carac = chars[i];

        if carac.is_ascii_digit() {
            return Ok(carac.to_digit(10).unwrap() as usize);
        }

        let maybe_three_carac = if left {
            chars.get(i..i + 3)
        } else {
            chars.get(i - 3..i)
        };
        if maybe_three_carac.is_none() {
            continue;
        }
        let three_carac = maybe_three_carac.unwrap().iter().collect::<String>();
        match three_carac.as_str() {
            "one" => return Ok(1),
            "two" => return Ok(2),
            "six" => return Ok(6),
            _ => (),
        }

        let maybe_four_carac = if left {
            chars.get(i..i + 4)
        } else {
            chars.get(i - 4..i)
        };
        if maybe_four_carac.is_none() {
            continue;
        }
        let four_carac = maybe_four_carac.unwrap().iter().collect::<String>();

        match four_carac.as_str() {
            "four" => return Ok(4),
            "five" => return Ok(5),
            "nine" => return Ok(9),
            _ => (),
        }

        let maybe_five_carac = if left {
            chars.get(i..i + 5)
        } else {
            chars.get(i - 5..i)
        };
        if maybe_five_carac.is_none() {
            continue;
        }
        let five_carac = maybe_four_carac.unwrap().iter().collect::<String>();

        match five_carac.as_str() {
            "three" => return Ok(3),
            "seven" => return Ok(7),
            "eight" => return Ok(8),
            _ => (),
        }
    }

    let error_msg = format!("Noo digit found in {}", String::from(input));

    return Err(error_msg);
}

fn get_calib_line_value(input: &str) -> usize {
    let left_digit = get_first_digit(input, true).unwrap();
    let right_digit = get_first_digit(input, false).unwrap();

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
