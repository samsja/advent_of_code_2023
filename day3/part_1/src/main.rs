use std::env;
use std::fs::File;
use std::io::prelude::*;

fn is_special(c: char) -> bool {
    return !(c.is_ascii_digit() || c == '.');
}

fn val_trio(top: &str, mid: &str, bottom: &str) -> usize {
    /*
    input like
    467..114..
    ...*......
    ..35..633.
     */

    let mid: Vec<char> = mid.chars().collect();
    let top: Vec<char> = top.chars().collect();
    let bottom: Vec<char> = bottom.chars().collect();

    let mut sum: usize = 0;

    let mut current_num: String = String::from("");

    let mut special_carac_around = false;

    for (i, &c) in mid.iter().enumerate() {
        special_carac_around =
            special_carac_around || is_special(c) || is_special(top[i]) || is_special(bottom[i]);

        print!("{special_carac_around} ");
        if c.is_ascii_digit() {
            current_num.push(c);
        } else {
            if current_num.len() > 0 {
                if special_carac_around {
                    let s = current_num.parse::<usize>().unwrap();
                    print!("{s} \n");
                    sum += s;
                }

                special_carac_around = false;
                current_num.clear();
            }
        }
    }

    sum
}

fn fake_concat_vec<'a>(input: &'a Vec<&str>, extra_line: &'a str, i: usize) -> &'a str {
    if i == 0 || i == input.len() + 1 {
        return extra_line;
    } else {
        return input[i - 1];
    }
}

fn game_sum(input: Vec<&str>) -> usize {
    let mut sum: usize = 0;
    let extra_line = ".".repeat(input[0].len());
    let extra_line = extra_line.as_str();

    let expanded_input_len = 2 + input.len();

    for i in 1..(expanded_input_len - 1) {
        let top = fake_concat_vec(&input, extra_line, i - 1);
        let mid = fake_concat_vec(&input, extra_line, i);
        let bottom = fake_concat_vec(&input, extra_line, i + 1);

        sum += val_trio(top, mid, bottom);
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
