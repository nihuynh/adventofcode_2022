use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut score = 0;
    if let Ok(lines) = read_lines("./src/data/day01.txt") {
        for line in lines {
            if let Ok(content) = line {
                println!("{}", content);
                score += count_points(&content);
            }
        }
        println!("The score is : {}", score);
    } else {
        println!("{}", "An error occured!");
    }
}

fn count_points(str: &str) -> u32 {
    match str {
        "A X" => 4,
        "A Y" => 8,
        "A Z" => 3,
        "B Y" => 1,
        "B Z" => 4,
        "B X" => 6,
        "C X" => 7,
        "C Y" => 2,
        "C Z" => 6,
        _ => panic!("Invalid move! {}", str),
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
