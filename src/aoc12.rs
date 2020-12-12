use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;



pub fn aoc12(){
    let mut arr: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines("aoc12input.txt") {
        for line in lines {
            if let Ok(l) = line {
                arr.push(l);
            }
        }
    }

    println!("Assignment 1: {}", assignment1(&arr));
    println!("Assignment 2: {}", assignment2(&arr));
}


fn assignment1(arr: &Vec<String>) -> i32 {
    //0 is east, 1 is south, 2 is west, 3 is north
    let mut direction = 0; 
    let mut pos_north = 0;
    let mut pos_east = 0;


    for line in arr {
        let command = line.chars().nth(0).unwrap();
        let value = line[1..line.len()].parse::<i32>().unwrap();

        match command {
            'N' => pos_north += value,
            'S' => pos_north -= value,
            'E' => pos_east += value,
            'W' => pos_east -= value,
            'L' => direction = (direction - (value / 90) + 4) % 4,
            'R' => direction = (direction + (value / 90) + 4) % 4,
            'F' => {
                match direction {
                    0 => pos_east += value,
                    1 => pos_north -= value,
                    2 => pos_east -= value,
                    3 => pos_north += value,
                    _ => continue,
                }
            }
            _ => continue,
        }
        // println!("Position ({}, {})", pos_east, pos_north);
    }
    
    return pos_east.abs() + pos_north.abs()
}

fn assignment2(arr: &Vec<String>) -> i32 {
    let mut pos_ship_north = 0;
    let mut pos_ship_east = 0;
    let mut pos_wp_north = 1;
    let mut pos_wp_east = 10;


    for line in arr {
        let command = line.chars().nth(0).unwrap();
        let value = line[1..line.len()].parse::<i32>().unwrap();

        match command {
            'N' => pos_wp_north += value,
            'S' => pos_wp_north -= value,
            'E' => pos_wp_east += value,
            'W' => pos_wp_east -= value,
            'L' => { let (a, b) = rotate_waypoint(pos_wp_north, pos_wp_east, value, -1);
                pos_wp_north = a; pos_wp_east = b;
            },
            'R' => { let (a, b) = rotate_waypoint(pos_wp_north, pos_wp_east, value, 1);
                pos_wp_north = a; pos_wp_east = b;
            },
            'F' => {
                pos_ship_east += pos_wp_east * value;
                pos_ship_north += pos_wp_north * value;
            }
            _ => continue,
        }
        // println!("Position ({}, {}), ({}, {})", pos_ship_east, pos_ship_north, pos_wp_east, pos_wp_north);
    }
    
    return pos_ship_east.abs() + pos_ship_north.abs()
}

fn rotate_waypoint(north : i32, east : i32, value: i32, direction : i32) -> (i32, i32) {
    let mut steps = value / 90;

    let mut new_north = north;
    let mut new_east = east;

    while steps > 0 {
        match direction {
            1 => {
                let tmp_north = new_north;
                let tmp_east = new_east;
                new_north = -tmp_east;
                new_east = tmp_north;
            },
            -1 => {
                let tmp_north = new_north;
                let tmp_east = new_east;
                new_north = tmp_east;
                new_east = -tmp_north;
            }
            _ => continue,
        }
        steps -= 1;
    }
    return (new_north,new_east);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
