use common::read_lines;

fn main() {
    let mut score = 0;
    if let Ok(lines) = read_lines("./src/data/day04.txt") {
        for line in lines {
            if let Ok(content) = line {
                println!("{}", content);
                score += 1;
            }
        }
        println!("The score is : {}", score);
    } else {
        println!("{}", "An error occured!");
    }
}
