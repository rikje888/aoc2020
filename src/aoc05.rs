
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
use regex::Regex;



pub fn aoc05(){

    let mut lines_array: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines("aoc05input.txt") {
        for line in lines {
            if let Ok(l) = line {
                //println!("{}", ip);
                lines_array.push(l);
            }
        }
    }

    let mut highestSeatID = 0;

    let mut seatIDs : Vec<i32> = Vec::new();

    for line in lines_array {


        let mut front_bound = 0;
        let mut back_bound = 127;
        let mut left_bound = 0;
        let mut right_bound = 7;

        for c in line.chars() {
            match c {
                'F' => back_bound = back_bound - (((back_bound - front_bound) as f32) / 2.0 + 0.5) as i32,
                'B' => front_bound = front_bound + (((back_bound - front_bound) as f32) / 2.0 + 0.5) as i32,
                'L' => right_bound = right_bound - (((right_bound - left_bound) as f32) / 2.0 + 0.5) as i32,
                'R' => left_bound = left_bound + (((right_bound - left_bound) as f32) / 2.0 + 0.5) as i32,
                _ => println!("AAAAAHHH")
            }

            
            
        }
        let seatID = front_bound * 8 + left_bound;
        seatIDs.push(seatID);
        println!("{} * 8 + {} = {}", front_bound, left_bound, seatID);

        if seatID > highestSeatID {
            highestSeatID = seatID;
        }

    }

    
    println!("Highest seatID {}", highestSeatID);

    seatIDs.sort();
    let mut counter = 32;
    for seatID in seatIDs {
        if seatID != counter {
            println!("AAh {} {}", counter, seatID);
        }
        counter = counter + 1;
    }

}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
