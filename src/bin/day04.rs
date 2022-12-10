use std::fs;

fn main() {
    let lines =
        fs::read_to_string("./src/data/day04.txt").expect("Should have been able to read the file");
    let split: Vec<String> = lines.split("\n\n").map(|s| s.to_string()).collect();
    if split.len() != 2 {
        panic!("Wrong input")
    };
    let mut map = parse_crates(&split[0]);
    let _instructions = parse_instructions(&split[1], &mut map);
}

fn parse_crates(input: &String) -> Vec<Vec<char>> {
    let mut map: Vec<String> = input.split("\n").map(|s| s.to_string()).collect();
    let header = map.pop();
    let size_map = (header.unwrap().len() + 1) / 4;
    let mut res: Vec<Vec<char>> = Vec::new();

    for _idx in 0..size_map {
        let tmp: Vec<char> = Vec::new();
        res.push(tmp);
    }
    // println!("Map of size {} : ", size_map);
    for line in map {
        parse_mapline(&line, &mut res);
    }
    for idx in 0..size_map {
        res[idx].reverse();
    }
    println!("{:#?}", res);
    res
}

fn parse_mapline(s: &String, stack: &mut Vec<Vec<char>>) {
    let line_iter = s
        .chars()
        .enumerate()
        .filter(|(idx, _c)| (*idx as i32 - 1) % 4 == 0);
    for (idx, c) in line_iter {
        let pile_idx: usize = ((idx as i32 - 1) / 4) as usize;
        // println!("Char {} at {}", c, pile_idx);
        match c {
            'A'..='Z' => stack[pile_idx].push(c),
            _ => {}
        }
    }
}

use parse_display::{Display, FromStr};
#[derive(Display, FromStr, PartialEq, Debug)]
#[display("move {count} from {origin} to {dest}")]
struct Operation {
    count: usize,
    origin: usize,
    dest: usize,
}

fn do_ope(ope: Operation, stacks: &mut Vec<Vec<char>>) {
    let mut grabed = Vec::new();
    for _ in 0..ope.count {
        let tmp = stacks[ope.origin - 1].pop().unwrap();
        grabed.push(tmp);
    }
    for _ in 0..ope.count {
        let tmp = grabed.pop().unwrap();
        stacks[ope.dest - 1].push(tmp);
    }
}

fn parse_instructions(input: &String, stacks: &mut Vec<Vec<char>>) {
    let size_stacks = stacks.len();
    println!("Instructions (size: {}): ", size_stacks);
    for line in input.split("\n") {
        let ope: Operation = line.trim().parse().unwrap();
        do_ope(ope, stacks);
        // println!("{:#?}", ope);
    }
    println!("{:#?}", stacks);
    for idx in 0..size_stacks {
        println!("{:#?}", stacks[idx].pop());
    }
}
