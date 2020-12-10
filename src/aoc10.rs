
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
use std::collections::HashSet;

use std::time::Instant;


const PREABLE_SIZE : usize = 25;

pub fn aoc10(){
    let start = Instant::now();
    let mut arr: Vec<i64> = Vec::new();
    if let Ok(lines) = read_lines("aoc10input.txt") {
        for line in lines {
            if let Ok(l) = line {
                arr.push(l.parse::<i64>().unwrap());
            }
        }
    }

    arr.sort();
    let mut previous = 0;
    let mut jolt_1_difference = 0;
    let mut jolt_3_difference = 1;
    for i in &arr {
        if i - previous == 1 {
            jolt_1_difference += 1;
        }
        else if i - previous == 3 {
            jolt_3_difference += 1;
        }
        previous = *i;
    }

    println!("{} * {} = {}", jolt_1_difference, jolt_3_difference, (jolt_1_difference * jolt_3_difference));


}





fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
