use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn aoc02(){

    let mut linesArray: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines("aoc02input.txt") {
        for line in lines {
            if let Ok(l) = line {
                //println!("{}", ip);
                linesArray.push(l);
            }
        }
    }

    let mut validPasswords = 0;
    for line in linesArray {
        let parts  = line.split(':').collect::<Vec<&str>>();
        let password = parts[1];
        let parts2 = parts[0].split(' ').collect::<Vec<&str>>();
        let character = parts2[1].chars().next().unwrap();
        let parts3 = parts2[0].split('-').collect::<Vec<&str>>();
        let min = parts3[0].parse::<i32>().unwrap();
        let max = parts3[1].parse::<i32>().unwrap();

        let mut counter = 0;
        for c in password.chars() {
            if c == character {
                counter = counter + 1
            }
        }
        if (counter >= min && counter <= max){
            validPasswords = validPasswords + 1;
        }

    }
    println!("Valid passwords: {}", validPasswords);

    let mut linesArray2: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines("aoc02input.txt") {
        for line in lines {
            if let Ok(l) = line {
                //println!("{}", ip);
                linesArray2.push(l);
            }
        }
    }

    validPasswords = 0;
    for line in linesArray2 {
        let parts  = line.split(':').collect::<Vec<&str>>();
        let password = parts[1];
        let parts2 = parts[0].split(' ').collect::<Vec<&str>>();
        let character = parts2[1].chars().next().unwrap();
        let parts3 = parts2[0].split('-').collect::<Vec<&str>>();
        let index1 = parts3[0].parse::<usize>().unwrap();
        let index2 = parts3[1].parse::<usize>().unwrap();

        let equalchar1 = password.chars().nth(index1).unwrap() == character;
        let equalchar2 = password.chars().nth(index2).unwrap() == character;

        if (equalchar1 && !equalchar2) || (!equalchar1 && equalchar2) {
            validPasswords = validPasswords + 1;
        }

    }
    println!("Valid passwords: {}", validPasswords)



}



fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
