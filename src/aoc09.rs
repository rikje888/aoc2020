
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
use std::collections::HashSet;

use std::time::Instant;


const PREABLE_SIZE : usize = 25;

pub fn aoc09(){
    let start = Instant::now();
    let mut arr: Vec<i64> = Vec::new();
    if let Ok(lines) = read_lines("aoc09input.txt") {
        for line in lines {
            if let Ok(l) = line {
                arr.push(l.parse::<i64>().unwrap());
            }
        }
    }

    let invalid_number = assignment1(&arr);
    println!("Assignment 1: {}", invalid_number);
    println!("Assignment 2: {}", assignment2(&arr, invalid_number));
    
    let duration = start.elapsed();
    println!("Elapsed time: {:?}", duration)
}

fn assignment1(arr : &Vec<i64>) -> i64 {
    for i in PREABLE_SIZE..arr.len() {
        let mut valid = false;
        for j in i - PREABLE_SIZE..i {
            let a = arr[j];
            let b = arr[i] - a;
            if a != b && arr[(i - PREABLE_SIZE) .. i].contains(&b){
                valid = true;
                break;
            }
        }
        if !valid {
            return arr[i];
        }
    }
    return 0;
}

fn assignment2(arr : &Vec<i64>, invalid_number : i64) -> i64 {
    for i in 0..arr.len() {
        let mut sum = 0;
        let mut j = i;
        let mut min = i64::MAX;
        let mut max = 0;
        while sum < invalid_number {
            sum += arr[j];
            min = if arr[j] < min { arr[j] } else { min };
            max = if arr[j] > max { arr[j] } else { max };
            j += 1;
        }
        if sum == invalid_number {
            return min + max;
        }
    }
    return 0;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
