// I tend to import stuff like
use std::{fs::File, io::{self, BufRead}, path::Path};
// that is, just one `use` per crate, but that's 100% test and zero
// real difference.

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // For reading the input, I'd go for reading the whole file into memory
    // Uless we are talking about files gigabytes in size, streamingd doesn't
    // really make much sense
    let input = std::fs::read_to_string("./inputs/day1.txt")?;
    for line in input.lines() {

    }
    // Alternatively, can go the unix way and read file from stdin:
    for line in std::io::stdin().lines() {

    }

    let mut elves = Vec::<i32>::new();
    let mut current_elf: i32 = 0;
    if let Ok(lines) = read_lines("./inputs/day1.txt") {
        for line in lines {
            if let Ok(ip) = line {
                if "".eq(&ip) { // this not Java, just `ip == ""` or, better, `if ip.is_empty()`
                    elves.push(current_elf);
                    current_elf = 0;
                } else {
                    current_elf += ip.parse::<i32>().unwrap();
                }
            }
        }
        elves.sort(); // n log n, but it could have been just n log k (for k greatest)
                      // If I were to optimize this, I'd tracked best elves in `[i32; 3]`
                      // (if there are only three of those folks)
                      // or in `std::collections::BinaryHeap` (if there are `k`). 
        println!("Three best elves carry {}", elves.iter().rev().take(3).sum::<i32>());
    }
}

// If let Ok swallows all the errors.
// You genreally want to return them instead using `?`
fn main() -> io::Result<()> {
    let lines = read_lines("./inputs/day1.txt")?; // <- note the ?
}
// With this, you'll run into a problem that `io::Error` and `ParseIntError` are different.
// So you want to use something like
fn main() -> Result<(), Box<dyn std::error::Error>>
// instead.

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
