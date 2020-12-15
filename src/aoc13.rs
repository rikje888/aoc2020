use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;



pub fn aoc13(){
    let mut arr: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines("aoc13input.txt") {
        for line in lines {
            if let Ok(l) = line {
                arr.push(l);
            }
        }
    }

    let timestamp = arr[0].parse::<i64>().unwrap();
    let busses_string = arr[1].split(",").collect::<Vec<&str>>();
    let mut busses: Vec<i64> = Vec::new();
    for bus_string in busses_string {
        match bus_string {
            "x" => busses.push(-1),
            _ => busses.push(bus_string.parse::<i64>().unwrap()),
        }
        
    }

    println!("Assignment 1: {}", assignment1(timestamp, &busses));
    println!("Assignment 2: {}", assignment2(&busses));
}

fn assignment1(_timestamp: i64, busses: &Vec<i64>) -> i64 {
    let mut timestamp = _timestamp;
    let mut bus_id = 0;
    'outerloop: 
    loop {
        for bus in busses {
            bus_id = *bus;
            if bus_id == -1 {
                continue;
            }
            if timestamp % bus_id == 0 {
                break 'outerloop;
            }
        }
        timestamp += 1;
    }
    return (timestamp - _timestamp) * bus_id;
}

fn assignment2_slow(busses: &Vec<i64>) -> i64 {
    let mut timestamp = 0;
    let max_value = busses.iter().max().unwrap();
    loop {
        let mut found_sequence = true;
        for i in 0..busses.len() {
            let bus_id = busses[i];
            if bus_id == -1 {
                continue;
            }
            if (timestamp + i as i64) % bus_id != 0 {
                found_sequence = false;
                break;
            }
        }

        if !found_sequence {
            timestamp += max_value;
            println!("x : {}", timestamp);
        }
        else {
            break;
        }
    }

    return timestamp;
}

fn assignment2(busses: &Vec<i64>) -> i64 {
    // let mut max_value : i64 = busses.iter().max().unwrap().next();
    let mut max_i = 0;

    
    let mut timestamp = 0;
    let mut timestamp_incrementer = busses[0];
    let mut busses_found = 1;
    while busses[busses_found] == -1 {
        busses_found += 1;
    }
    loop {
        if timestamp % busses[busses_found] == (busses[busses_found] - (busses_found as i64 % busses[busses_found])){
            busses_found += 1;
            if busses_found >= busses.len(){
                return timestamp;
            }
            timestamp_incrementer = 1;
            for i in 0..busses_found{
                if busses[i] as i64 != -1 {
                    timestamp_incrementer *= busses[i];
                }
            }
            
            while busses[busses_found] == -1 {
                busses_found += 1;
            }
            
            // println!("Timestamp: {}", timestamp);
        }
        timestamp += timestamp_incrementer;
    }

}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
