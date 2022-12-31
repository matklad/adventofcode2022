use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// Tuples! (i32, i32) or [i32; 2] would be much more efficient than `Vec<i32>`,
// as well as more type safe.
fn contains(x: &Vec<i32>, y: &Vec<i32>) -> bool {
    (x[0] <= y[0]) & (x[1] >= y[1]) // &&, & is a bitwise or
}

fn overlap(x: &Vec<i32>, y: &Vec<i32>) -> bool {
    !((x[1] < y[0]) | (y[1] < x[0])) // ||
}

fn range(v: &str) -> Vec<i32> {
    let (x, y) = v.split_once("-").unwrap();
    (x.parse().unwrap(), y.parse().unwrap())

    v.split("-").map(|s| s.parse().expect("parse error")).collect::<Vec<i32>>()
}

fn day4(lines: std::io::Lines<io::BufReader<File>>) {
    let mut result: i32 = 0;// for counting things you generally want unsigned, `let mut result = 0u32`
    for line in lines {
        if let Ok(ip) = line {
            let vals: Vec<&str> = ip.split(",").collect();
            let range1: Vec<i32> = range(vals[0]);
            let range2: Vec<i32> = range(vals[1]);
            if contains(&range1, &range2) | contains(&range2, &range1) {
                result += 1; // could write the whole thing as an iterator chain with `.fiter().count()` at the end.
            }
        }
    }
    println!("Result: {}", result);
}


fn day4_part2(lines: std::io::Lines<io::BufReader<File>>) {
    let mut result: i32 = 0;
    for line in lines {
        if let Ok(ip) = line {
            let vals: Vec<&str> = ip.split(",").collect();
            let range1: Vec<i32> = range(vals[0]);
            let range2: Vec<i32> = range(vals[1]);
            if overlap(&range1, &range2) {
                result += 1;
            }
        }
    }
    println!("Result: {}", result);
}


fn main() {
    if let Ok(lines) = read_lines("./inputs/day4.txt") {
        day4_part2(lines);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
