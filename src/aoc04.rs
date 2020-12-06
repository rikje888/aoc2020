
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
use regex::Regex;



pub fn aoc04(){

    let mut lines_array: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines("aoc04input.txt") {
        for line in lines {
            if let Ok(l) = line {
                //println!("{}", ip);
                lines_array.push(l);
            }
        }
    }


    
    let mut i = 0;
    let mut valid_passport = 0;
    let mut passport: HashMap<String, String> = HashMap::new();
    while i < lines_array.len(){

        let line: &String = &lines_array[i];
        match line.as_str() {
            "" => {
                if check_validity(passport){
                    valid_passport += 1
                }
                passport = HashMap::new();
            },
            _ => {
                let parts = line.split(" ");
                for part in parts{
                    let key = part.split(":").collect::<Vec<&str>>()[0];
                    let value = part.split(":").collect::<Vec<&str>>()[1];
                    passport.insert(key.to_string(), value.to_string());
                    println!("Key: {} value: {}", key, value)
                }
            }
        }
        if i > lines_array.len(){
            break;
        }
        i = i + 1;
        
        

    }
    println!("Number of valid passports: {}", valid_passport)
}

fn check_validity(passport: HashMap<String, String>) -> bool {
    if passport.contains_key("byr") &&
        passport.contains_key("iyr") &&
        passport.contains_key("eyr") &&
        passport.contains_key("hgt") &&
        passport.contains_key("hcl") &&
        passport.contains_key("ecl") &&
        passport.contains_key("pid")
    {
        if check_year(passport.get("byr").unwrap().to_string(), 1920, 2002) &&
            check_year(passport.get("iyr").unwrap().to_string(), 2010, 2020) &&
            check_year(passport.get("eyr").unwrap().to_string(), 2020, 2030) &&
            check_height(passport.get("hgt").unwrap().to_string()) &&
            check_haircolor(passport.get("hcl").unwrap().to_string()) &&
            check_eyecolor(passport.get("ecl").unwrap().to_string()) &&
            check_pid(passport.get("pid").unwrap().to_string()) {
                println!("VALID");
                return true;
            }
    }
    println!("INVALID");
    return false;
}

fn check_year(year: String, min : i32, max : i32) -> bool {
    if year.chars().count() != 4 {
        return false;
    }
    let int_year = year.parse::<i32>().unwrap();
    if int_year < min || int_year > max {
        return false
    }

    return true;
}

fn check_height(height: String) -> bool {
    if !height.contains("cm") && !height.contains("in") {
        return false
    }
    let int_height = height[0..height.len() - 2].parse::<i32>().unwrap();
    if height.contains("cm") {
        if int_height < 150 || int_height > 193{
            return false;
        }
    }
    if height.contains("in") {
        if int_height < 59 || int_height > 76{
            return false;
        }
    }
    return true;
}

fn check_haircolor(color: String) -> bool {
    let re = Regex::new(r"#[a-f0-9]{6}").unwrap();
    return re.is_match(&color);
}

fn check_eyecolor(color: String) -> bool {
    let re = Regex::new(r"amb|blu|brn|gry|grn|hzl|oth").unwrap();
    return re.is_match(&color) && color.len() == 3;
}

fn check_pid(pid: String) -> bool {
    let re = Regex::new(r"[0-9]{9}").unwrap();
    return re.is_match(&pid) && pid.len() == 9;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
