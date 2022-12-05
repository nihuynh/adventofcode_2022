use common::read_lines;

fn main() {
    let mut score = 0;
    let mut score_star2 = 0;
    let mut idx = 0;
    let mut group: [String; 3] = Default::default();

    if let Ok(lines) = read_lines("./src/data/day02.txt") {
        for line in lines {
            if let Ok(content) = line {
                // println!("{}", content);
                score += read_and_compute(&content);
                group[idx] = content;
                idx += 1;
                if idx == 3 {
                    idx = 0;
                    let badge = find_badge(&group[0], &group[1], &group[2]);
                    score_star2 += convert_to_score(badge);
                    println!("1: {} 2: {} 3: {}", group[0], group[1], group[2]);
                }
            }
        }
        println!(
            "The score is : {} and score star2 is : {}",
            score, score_star2
        );
    } else {
        println!("{}", "An error occured!");
    }
}

fn find_badge(sack1: &str, sack2: &str, sack3: &str) -> char {
    let a: Vec<char> = sack1.chars().filter(|c| sack2.contains(*c)).collect();
    let b: Vec<char> = sack3.chars().filter(|c| sack2.contains(*c)).collect();
    let res: Vec<&char> = a.iter().filter(|c| b.contains(*c)).collect();

    println!("res: {:#?}", res[0]);
    *res[0]
}

fn read_and_compute(str: &str) -> i32 {
    let len = str.len();
    if len % 2 == 1 {
        panic!("Invalid input string!");
    }
    let matching_item = find_matching(&str[0..len / 2], &str[len / 2..len]);
    convert_to_score(matching_item)
}

fn find_matching(sack1: &str, sack2: &str) -> char {
    for c in sack1.chars() {
        if sack2.contains(c) {
            println!(
                "sack1: {} sack2: {} matching C: {} [{}]",
                sack1,
                sack2,
                c,
                convert_to_score(c)
            );
            return c;
        }
    }
    panic!("Invalid output");
}

fn convert_to_score(c: char) -> i32 {
    match c {
        'a'..='z' => c as i32 - 96,
        'A'..='Z' => c as i32 - 38,
        _ => panic!("Invalid char [{}]", c),
    }
}
