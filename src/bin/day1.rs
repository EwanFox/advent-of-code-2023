use std::fs;
fn get_puzzle_input() -> String {
    fs::read_to_string("./src/inputs/day1.txt").expect("Unable to read input!")
}

fn replace_english_digits(s: &str) -> String {
    let english_digits = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "zero"];

    let mut result = s.to_string();

    for (i, digit) in english_digits.iter().enumerate() {
        result = result.replace(digit, format!("{}{}{}",english_digits[i],&(i + 1).to_string(),english_digits[i]).as_str());
    }

    result
}

fn main() {
    let input: String = get_puzzle_input();
    let lines = input.lines();
    let mut calibration_numbers = Vec::new();
    let mut sum: i32 = 0;
    for line in lines {
        let parsed = replace_english_digits(line);
        println!("{} -> {}",line,parsed);
        let e: Vec<char> = parsed.chars().into_iter().filter(|f| f.is_numeric()).collect();
        let mut cal_string = String::new();
        cal_string.push(e[0]);
        cal_string.push(e[e.len()-1]);
        println!("{}",cal_string);
        calibration_numbers.push(cal_string.parse::<i32>().unwrap());
    }
    for num in calibration_numbers {
        sum = sum + num;
    }
    println!("{}",sum);
}

