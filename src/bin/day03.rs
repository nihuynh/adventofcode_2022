// struct Job {
//     start: i32,
//     end: i32,
// }

// impl Job {
//     fn parse() -> Self {
//         Job { start: 30, end: 32 }
//     }
// }

use common::read_lines;

fn main() {
    let mut score = 0;
    if let Ok(lines) = read_lines("./src/data/day03_test.txt") {
        for line in lines {
            if let Ok(content) = line {
                println!("{}", content);
                let pair: Vec<&str> = content.split(',').collect();
                if check_overlap(pair) {
                    score += 1;
                }
                // let job: Vec<Vec<char>> = pair.iter().map(|)split('-').collect;
            }
        }
        println!("The score is : {}", score);
    } else {
        println!("{}", "An error occured!");
    }
}

fn check_overlap(pair: Vec<&str>) -> bool {
    let a: Vec<&str> = pair[0].split('-').collect();
    let b: Vec<&str> = pair[1].split('-').collect();
    println!("A: {:#?} and B: {:#?}", a, b);
    // let process_a: Vec<i32> = a.iter().map(|s| s[0] as i32).collect();
    // println!("data : {:#?}", process_a);
    true
}
