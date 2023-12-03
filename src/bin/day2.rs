use std::fs;
fn get_puzzle_input() -> String {
    fs::read_to_string("./src/inputs/day2.txt").expect("Unable to read input!")
}


fn main() {
    let input: String = get_puzzle_input();
    let lines = input.lines();
    let mut ids: Vec<i32> = Vec::new();
    for line in lines {
        let game: &str = line.split(":").collect::<Vec<&str>>()[1];
        let draws = game.split(";").collect::<Vec<&str>>();
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;
        for draw in draws {
            let colours = draw.split(",").collect::<Vec<&str>>();
            for colour in colours {                
                let parts: Vec<&str> = colour.split_whitespace().collect::<Vec<&str>>();
                let letter = parts[1];
                let amount = parts[0];
                match letter {
                    "green" => {
                        if amount.parse::<i32>().unwrap() > g{
                            g = amount.parse::<i32>().unwrap();
                        }
                    }
                    "red" => {
                        if amount.parse::<i32>().unwrap() > r{
                            r = amount.parse::<i32>().unwrap();
                        }
                    }
                    "blue" => {
                        if amount.parse::<i32>().unwrap() > b{
                            b = amount.parse::<i32>().unwrap();
                        }
                    }
                    _ => {
                        
                    }
                }
            }
        }
        ids.push(r*g*b);
    }
    let mut sum: i64 = 0;
    for id in ids {
        println!("{}", id);
        sum += id as i64;
    }
    println!("{}", sum);

}

