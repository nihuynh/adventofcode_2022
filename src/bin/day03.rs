use common::read_lines;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>()
    };
}

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
    let a_num: Vec<i32> = a.iter().map(|s| parse_input!(s, i32).unwrap()).collect();
    let b_num: Vec<i32> = b.iter().map(|s| parse_input!(s, i32).unwrap()).collect();
    // println!("A : {:#?} B : {:#?}", a_num, b_num);
    // First star:
    // (is_in_range(a_num[0], b_num[0], b_num[1]) && is_in_range(a_num[1], b_num[0], b_num[1]))
    //     || (is_in_range(b_num[0], a_num[0], a_num[1]) && is_in_range(b_num[1], a_num[0], a_num[1]))
    // Second star:
    (is_in_range(a_num[0], b_num[0], b_num[1]) || is_in_range(a_num[1], b_num[0], b_num[1]))
        || (is_in_range(b_num[0], a_num[0], a_num[1]) || is_in_range(b_num[1], a_num[0], a_num[1]))
}

fn is_in_range(value: i32, min: i32, max: i32) -> bool {
    value >= min && value <= max
}
