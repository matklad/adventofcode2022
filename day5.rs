use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn parse_stacks(vec: &mut Vec<String>) -> Vec<Vec<String>> {
    let columns: Vec<i32> = vec.pop().expect("pop error")
                .trim().split_whitespace()
                .map(|s| s.parse().expect("parse error"))
                .collect();
    println!("columns: {}", columns.len());
    let mut result = Vec::<Vec<String>>::new();
    for _ in 0..columns.len() {
        result.push(Vec::<String>::new());
    }
    for v in vec.into_iter().rev() {
        for i in 0..columns.len() {
            let c = v.chars().nth(4*i + 1).unwrap().to_string();
            if !" ".eq(&c) {
                result[i].push(c);
            }
        }
    }
    for sub_vec in &result {
        print!("{:?}\n", sub_vec);
    }
    result
}

fn parse_instructions(s: &str) -> Vec<i32> { // again, want to use (u32, u32, u32) here to avoid allocations
    s.replace("move ", "").replace("from ", "").replace("to ", "")
        .split_whitespace()
        .map(|s| s.parse().expect("parse error"))
        .collect()
}


fn day5(lines: std::io::Lines<io::BufReader<File>>) {
    let mut strings = Vec::<String>::new();
    let mut stacks = Vec::<Vec<String>>::new();
    let mut to_collect: bool = true;
    for line in lines {
        if let Ok(ip) = line {
            if to_collect & !"".eq(&ip) {
                strings.push(ip.clone());
            }
            if to_collect & "".eq(&ip) {
                to_collect = false;
                stacks = parse_stacks(&mut strings);
            } else if !to_collect {
                let ins = parse_instructions(&ip);
                let from = stacks[(ins[1] - 1) as usize].clone();
                stacks[(ins[2] - 1) as usize].extend_from_slice(&from[from.len()-ins[0] as usize..]);
                stacks[(ins[1] - 1) as usize].drain(from.len()-ins[0] as usize..);

                // Part 1

                // for i in 0..ins[0] {
                //     let v = stacks[(ins[1] - 1) as usize].pop().unwrap();
                //     stacks[(ins[2] - 1) as usize].push(v);
                // }
            }
        }
    }
    // Print the contents of the stacks
    for sub_vec in &stacks {
        print!("{sub_vec:?}\n");
    }
    println!("{}", stacks.into_iter().map(|v| v[v.len() - 1].clone()).collect::<String>());

}

fn main() {
    if let Ok(lines) = read_lines("./inputs/day5.txt") {
        day5(lines);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
