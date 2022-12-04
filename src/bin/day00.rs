use std::cmp::Reverse;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>()
    };
}

fn main() {
    let mut max = 0;
    let mut sum = 0;
    let mut trunk = Vec::new();
    if let Ok(lines) = read_lines("./src/data/day00.txt") {
        for line in lines {
            if let Ok(content) = line {
                // println!("{}", content);
                match content.len() {
                    0 => {
                        if sum > max {
                            max = sum;
                        };
                        trunk.push(sum);
                        sum = 0;
                    }
                    _ => {
                        let value = parse_input!(content, i32).unwrap();
                        sum += value;
                    }
                }
            }
        }
        println!("The maximum of calories found was : {}", max);
        trunk.sort();
        trunk.reverse();
        println!(
            "The top three hold in total : {}",
            trunk[0] + trunk[1] + trunk[2]
        );
    } else {
        println!("{}", "An error occured!");
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
