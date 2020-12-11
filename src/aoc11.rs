use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn aoc11(){
    let mut arr: Vec<Vec<char>> = Vec::new();
    if let Ok(lines) = read_lines("aoc11input.txt") {
        for line in lines {
            if let Ok(l) = line {
                let mut a: Vec<char> = Vec::new();
                for c in l.chars(){
                    a.push(c);
                }
                
                arr.push(a);
            }
        }
    }

    

    println!("Assignment 1: {}", assignment1(&arr));
    println!("Assignment 2: {}", assignment2(&arr));
    

}

fn assignment1(_arr : &Vec<Vec<char>>) -> i32 {
    let mut arr: Vec<Vec<char>> = _arr.to_vec();
    let mut number_of_iterations = 0;
    loop {
        let mut newArr: Vec<Vec<char>> = arr.clone();
        for y in 0..arr.len() {
            for x in 0..arr[y].len() {
                match arr[y][x] {
                    'L' =>  if get_number_of_occupied_adjacent_seats(x, y, &arr) == 0 { newArr[y][x] = '#'; },
                    '#' => if get_number_of_occupied_adjacent_seats(x, y, &arr) >= 4 { newArr[y][x] = 'L'; },
                    _=> newArr[y][x] = '.',
                }
            }
        }
        let equal = are_vectors_equal(&arr, &newArr);
        arr = newArr.clone();
        if equal {
            break;
        }
        number_of_iterations += 1;

    }
    return count_number_of_taken_seats(&arr);
}

fn assignment2(_arr : &Vec<Vec<char>>) -> i32 {
    let mut arr: Vec<Vec<char>> = _arr.to_vec();
    let mut number_of_iterations = 0;
    loop {
        let mut newArr: Vec<Vec<char>> = arr.clone();
        for y in 0..arr.len() {
            for x in 0..arr[y].len() {
                match arr[y][x] {
                    'L' =>  if get_number_of_occupied_first_seats(x, y, &arr) == 0 { newArr[y][x] = '#'; },
                    '#' => if get_number_of_occupied_first_seats(x, y, &arr) >= 5 { newArr[y][x] = 'L'; },
                    _=> newArr[y][x] = '.',
                }
            }
        }
        let equal = are_vectors_equal(&arr, &newArr);
        arr = newArr.clone();
        if equal {
            break;
        }
        number_of_iterations += 1;

    }
    return count_number_of_taken_seats(&arr);
}



fn are_vectors_equal(arrA : &Vec<Vec<char>>, arrB : &Vec<Vec<char>>) -> bool {
    let mut equal = true;
    for y in 0..arrA.len() {
        for x in 0..arrA[y].len() {
            if arrA[y][x] != arrB[y][x] {
                equal = false;
            }
        }
    }
    return equal;
}

fn count_number_of_taken_seats(arr : &Vec<Vec<char>>) -> i32 {
    let mut number_of_taken_seats = 0;
    for y in 0..arr.len() {
        for x in 0..arr[y].len() {
            if arr[y][x] == '#' {
                number_of_taken_seats += 1;
            }
        }
    }
    return number_of_taken_seats;
}


fn get_number_of_occupied_adjacent_seats(x : usize, y : usize, arr : &Vec<Vec<char>>) -> i8 {
    let mut occupied_seats = 0;
    if x > 0 &&                             arr[y][x - 1] == '#' { occupied_seats += 1 }
    if x > 0 && y > 0 &&                    arr[y - 1][x - 1] == '#' { occupied_seats += 1 }
    if y > 0 && y > 0 &&                    arr[y - 1][x] == '#' { occupied_seats += 1 }
    if x < arr[y].len() - 1 && y > 0 &&         arr[y - 1][x + 1] == '#' { occupied_seats += 1 }
    if x < arr[y].len() - 1 &&                  arr[y][x + 1] == '#' { occupied_seats += 1 }
    if x < arr[y].len() - 1 && y < arr.len() - 1 && arr[y + 1][x + 1] == '#' { occupied_seats += 1 }
    if y < arr.len() - 1 &&                     arr[y + 1][x] == '#' { occupied_seats += 1 }
    if x > 0 && y < arr.len() - 1 &&            arr[y + 1][x - 1] == '#' { occupied_seats += 1 }
    return occupied_seats;
}

fn get_number_of_occupied_first_seats(x : usize, y : usize, arr : &Vec<Vec<char>>) -> i8 {
    let mut occupied_seats = 0;
    if first_seat_in_dir(x, y, 0, 1, arr) == '#' { occupied_seats += 1}
    if first_seat_in_dir(x, y, 1, 1, arr) == '#' { occupied_seats += 1}
    if first_seat_in_dir(x, y, 1, 0, arr) == '#' { occupied_seats += 1}
    if first_seat_in_dir(x, y, 1, -1, arr) == '#' { occupied_seats += 1}
    if first_seat_in_dir(x, y, 0, -1, arr) == '#' { occupied_seats += 1}
    if first_seat_in_dir(x, y, -1, -1, arr) == '#' { occupied_seats += 1}
    if first_seat_in_dir(x, y, -1, 0, arr) == '#' { occupied_seats += 1}
    if first_seat_in_dir(x, y, -1, 1, arr) == '#' { occupied_seats += 1}
    return occupied_seats;
}


fn first_seat_in_dir(x: usize, y: usize, dir_x: i8, dir_y: i8, arr : &Vec<Vec<char>>) -> char {

    let mut pos_x : i8 = x as i8;
    let mut pos_y : i8 = y as i8;
    let mut first_seat = '.';
    loop {
        pos_x += dir_x;
        pos_y += dir_y;
        if pos_x >= 0 && pos_x < arr[y].len() as i8 && pos_y >= 0 && pos_y < arr.len() as i8 {
            match arr[pos_y as usize][pos_x as usize] {
                '.' => continue,
                _ => {
                    first_seat = arr[pos_y as usize][pos_x as usize];
                    break;
                }
            }
        }
        else {
            break;
        }
    }
    return first_seat;
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
