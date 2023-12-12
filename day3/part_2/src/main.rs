use std::env;
use std::fs::File;
use std::io::prelude::*;

fn is_special(c: char) -> bool {
    return !(c.is_ascii_digit() || c == '.');
}

fn get_expanded_line(line: &str) -> Vec<char> {
    ".".chars().chain(line.chars()).chain(".".chars()).collect()
}
fn find_num_and_push_it(nums: &mut Vec<usize>, line: &Vec<char>, i: usize) {
    let mut num_left = String::from("");

    let mut j = i - 1;
    let mut c: char = line[j];
    while c.is_ascii_digit() {
        num_left.push(c);
        j -= 1;
        c = line[j];
    }

    num_left = num_left.chars().rev().collect();

    let mut num_right = String::from("");

    let mut j = i + 1;

    c = line[j];
    while c.is_ascii_digit() {
        num_right.push(c);
        j += 1;
        c = line[j];
    }

    c = line[i];
    if c.is_ascii_digit() {
        let final_num = format!("{num_left}{c}{num_right}");
        nums.push(final_num.parse::<usize>().unwrap())
    } else {
        if !num_left.is_empty() {
            nums.push(num_left.parse::<usize>().unwrap())
        }

        if !num_right.is_empty() {
            nums.push(num_right.parse::<usize>().unwrap())
        }
    }
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

    for i in 1..(mid.len() - 1) {
        if is_special(mid[i]) {
            let mut nums: Vec<usize> = vec![];

            find_num_and_push_it(&mut nums, &mid, i);
            find_num_and_push_it(&mut nums, &top, i);
            find_num_and_push_it(&mut nums, &bottom, i);

            if nums.len() == 2 {
                sum += nums[0] * nums[1];
                // println!("{} {} {}",mid[i], nums[0], nums[1]);
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
