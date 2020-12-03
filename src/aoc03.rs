use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn aoc03(){

    let mut lines_array: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines("aoc03input.txt") {
        for line in lines {
            if let Ok(l) = line {
                //println!("{}", ip);
                lines_array.push(l);
            }
        }
    }

    let number_of_lines : usize = lines_array.len();
    let number_of_characters : usize = lines_array[0].chars().count();

    let mut map : Vec<Vec<u8>> = vec![vec![0u8; number_of_lines]; number_of_characters];

    let mut y = 0;
    for line in lines_array {
        let mut x = 0;
        for c in line.chars() {
            if c == '.' {
                map[x][y] = 0;
            }
            else if c == '#' {
                map[x][y] = 1;
            }
            x = x + 1
        }
        y = y + 1
    }

    println!("Assignment 1: {}", traverse_map(&map, 3,1));
    println!("Assignment 2: {}", 
            i64::from(traverse_map(&map, 1,1)) *
            i64::from(traverse_map(&map, 3,1)) *
            i64::from(traverse_map(&map, 5,1)) *
            i64::from(traverse_map(&map, 7,1)) *
            i64::from(traverse_map(&map, 1,2))
        );


}

fn traverse_map(map : &Vec<Vec<u8>>, step_x : usize, step_y : usize) -> i32{
    let mut my_pos_x : usize = 0;
    let mut my_pos_y : usize = 0;
    let mut number_of_trees_encountered : i32 = 0;
    loop {
        number_of_trees_encountered += i32::from(map[my_pos_x][my_pos_y]);
        my_pos_x = (my_pos_x + step_x) % map.len();
        my_pos_y += step_y;
        if my_pos_y >= map[0].len() {
            break
        }
    }
    return number_of_trees_encountered
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
