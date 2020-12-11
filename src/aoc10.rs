
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn aoc10(){
    let mut arr: Vec<i64> = Vec::new();
    if let Ok(lines) = read_lines("aoc10input.txt") {
        for line in lines {
            if let Ok(l) = line {
                arr.push(l.parse::<i64>().unwrap());
            }
        }
    }
    arr.push(0);
    arr.sort();

    println!("Assignment 1: {}", assignment1(&arr));
    println!("Assignment 2: {}", assignment2(&arr));
}

fn assignment1(arr : &Vec<i64>) -> i64 {
    let mut previous = 0;
    let mut jolt_1_difference = 0;
    let mut jolt_3_difference = 1;
    for i in arr {
        if i - previous == 1 {
            jolt_1_difference += 1;
        }
        else if i - previous == 3 {
            jolt_3_difference += 1;
        }
        previous = *i;
    }
    return jolt_1_difference * jolt_3_difference;
}

fn assignment2(arr : &Vec<i64>) -> i64 {
    let mut arrangements : i64 = 1;
    let mut i = 0;
    loop {
        let mut j = i;
        loop {
            if j + 1 >= arr.len(){
                break;
            }
            if arr[j] + 1 != arr[j + 1] {
                break;
            }
            j += 1;
        }
        if i != j {
            match j - i + 1 {
                1 => arrangements = arrangements * 1,
                2 => arrangements = arrangements * 1,
                3 => arrangements = arrangements * 2,
                4 => arrangements = arrangements * 4,
                5 => arrangements = arrangements * 7,
                _ => break,
            }
            i = j;
        }
        else {
            i += 1;
        }
        if i + 1 >= arr.len(){
            break;
        }
    }
    return arrangements;
}



fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
