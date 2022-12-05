use common::read_lines;

fn main() {
    let mut score = 0;
    if let Ok(lines) = read_lines("./src/data/day01.txt") {
        for line in lines {
            if let Ok(content) = line {
                // println!("{}", content);
                score += count_points(&content, 2);
            }
        }
        println!("The score is : {}", score);
    } else {
        println!("{}", "An error occured!");
    }
}

fn count_points(str: &str, mode: i32) -> u32 {
    if mode == 1 {
        match str {
            "A X" => 4,
            "A Y" => 8,
            "A Z" => 3,
            "B X" => 1,
            "B Y" => 5,
            "B Z" => 9,
            "C X" => 7,
            "C Y" => 2,
            "C Z" => 6,
            _ => panic!("Invalid move! {}", str),
        }
    } else {
        match str {
            "A X" => 3,
            "A Y" => 4,
            "A Z" => 8,
            "B X" => 1,
            "B Y" => 5,
            "B Z" => 9,
            "C X" => 2,
            "C Y" => 6,
            "C Z" => 7,
            _ => panic!("Invalid move! {}", str),
        }
    }
}
