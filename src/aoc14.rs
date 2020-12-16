use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;



pub fn aoc14(){
    let mut arr: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines("aoc14input.txt") {
        for line in lines {
            if let Ok(l) = line {
                arr.push(l);
            }
        }
    }


    
    println!("Assignment 1: {}", assignment1(&arr));
    println!("Assignment 2: {}", assignment2(&arr));
    

}

fn process_bit_mask(line: &String) -> (i64, i64, i64) {
    let mask_string = line.split(" ").collect::<Vec<&str>>()[2].to_string();
    let mut mask_ones : i64 = 0;
    let mut mask_zeros : i64 = i64::MAX;
    let mut mask_floating : i64 = 0;
    let mut i = 0;
    for c in mask_string.chars().rev(){
        match c {
            '1' => mask_ones |= 1 << i,
            '0' => mask_zeros &= !(1 << i),
            'X' => mask_floating |= 1 << i,
            _ => (),
        }
        i += 1;
        
    }
    // println!("Ones: {:#036b}", mask_ones);
    // println!("Zeros: {:#036b}", mask_zeros);
    return (mask_ones, mask_zeros, mask_floating);
}

fn process_mem_set(line: &String, mask_ones :i64, mask_zeros :i64) -> (i64, i64){
    let memaddress = line.split("[").collect::<Vec<&str>>()[1].split("]").collect::<Vec<&str>>()[0].parse::<i64>().unwrap();
    let mut value = line.split(" ").collect::<Vec<&str>>()[2].parse::<i64>().unwrap();
    value |= mask_ones;
    value &= mask_zeros;
    // println!("Write: {:#036b} to {}", value, memaddress);
    return (memaddress, value);
}




fn assignment1(arr: &Vec<String>) -> i64 {
    let mut mask_ones = 0;
    let mut mask_zeros = 0;
    let mut memory: HashMap<i64, i64> = HashMap::new();
    for line in arr {
        if line.contains("mask") {
            let (_mask_ones, _mask_zeros, _mask_floating) = process_bit_mask(&line);
            mask_ones = _mask_ones;
            mask_zeros = _mask_zeros;
        }
        else if line.contains("mem") {
            let (memaddress, value) = process_mem_set(line, mask_ones, mask_zeros);
            
            memory.insert(memaddress, value);
        }
    }
    
    let mut sum = 0;
    for (_memaddress, value) in &memory {
        sum += value;
    }
    return sum;
}


fn assignment2(arr: &Vec<String>) -> i64 {
    let mut mask_ones = 0;
    let mut mask_floating = 0;
    let mut memory: HashMap<i64, i64> = HashMap::new();
    for line in arr {
        if line.contains("mask") {
            let (_mask_ones, _mask_zeros, _mask_floating) = process_bit_mask(&line);
            mask_ones = _mask_ones;
            mask_floating = _mask_floating;
        }
        else if line.contains("mem") {
            let mut memaddress = line.split("[").collect::<Vec<&str>>()[1].split("]").collect::<Vec<&str>>()[0].parse::<i64>().unwrap();
            let value = line.split(" ").collect::<Vec<&str>>()[2].parse::<i64>().unwrap();

            memaddress |= mask_ones;

            let mut mask_floating_copy = mask_floating;
            let mut number_of_floating_digits = 0; 
            while mask_floating_copy > 0 { 
                number_of_floating_digits += mask_floating_copy & 1; 
                mask_floating_copy >>= 1; 
            } 


            memaddress &= !mask_floating;
            for i in 0..2u32.pow(number_of_floating_digits as u32) {
                let mut memaddress_copy = memaddress;
                let mut number_to_write = i;
                let mut digit_to_write = 0;
                let mut digit_in_mask = 0;
                while number_to_write > 0 {
                    if mask_floating & (1 << digit_in_mask) > 0 {
                        if number_to_write & (1 << 0) == 1 {
                            // println!("Write a 1 to the {}th digit in memaddress", digit_in_mask);
                            memaddress_copy |= 1 << digit_in_mask;

                        }
                        else {
                            // println!("Write a 0 to the {}th digit in memaddress", digit_in_mask);
                            memaddress_copy &= !(1 << digit_in_mask);
                        }
                        number_to_write >>= 1;
                        digit_to_write += 1;
                    }
                    digit_in_mask += 1;
                }
                memory.insert(memaddress_copy, value);
            }
        }
    }
    
    let mut sum = 0;
    for (_memaddress, value) in &memory {
        sum += value;
    }
    return sum;
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
