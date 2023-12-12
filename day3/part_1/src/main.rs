use std::env;
use std::fs::File;
use std::io::prelude::*;

fn is_special(c: char) -> bool {
    return !(c.is_ascii_digit() || c == '.');
}

fn get_expanded_line(line: &str) -> Vec<char> {
    line.chars().chain(".".chars()).collect()
}
fn val_trio(top: &str, mid: &str, bottom: &str) -> usize {
    /*
    input like
    467..114..
    ...*......
    ..35..633.
     */

    // println!("{mid}");

    let top: Vec<char> = get_expanded_line(top);
    let mid: Vec<char> = get_expanded_line(mid);
    let bottom: Vec<char> = get_expanded_line(bottom);

    let mut sum: usize = 0;

    let mut current_num: String = String::from("");
    let mut special_carac_before = false;

    let mut special_carac_this_line: bool;

    for (i, &c) in mid.iter().enumerate() {
        special_carac_this_line = is_special(bottom[i]) || is_special(mid[i]) || is_special(top[i]);

        if c.is_ascii_digit() {
            current_num.push(c);
            special_carac_before = special_carac_before || special_carac_this_line;
        } else {
            if !current_num.is_empty() {
                if special_carac_before || special_carac_this_line {
                    let s = current_num.parse::<usize>().unwrap();
                    // print!("{s} \n");
                    sum += s;
                }
                current_num.clear();
            }

            special_carac_before = special_carac_this_line
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
