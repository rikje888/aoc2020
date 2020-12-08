
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
use std::collections::HashSet;

struct Instruction {
    command: String,
    value: i32
}

pub fn aoc08(){

    let mut instructions: Vec<Instruction> = Vec::new();
    if let Ok(lines) = read_lines("aoc08input.txt") {
        for line in lines {
            if let Ok(l) = line {
                //println!("{}", ip);
                //lines_array.push(l);
                let parts = l.split(" ").collect::<Vec<&str>>();

                let i = Instruction {
                    command: parts[0].to_string(),
                    value: parts[1].parse::<i32>().unwrap()
                };
                instructions.push(i);


            }
        }
    }


    let mut accumulator = 0;
    let mut instruction_pointer :i32 = 0;

    let mut instructions_passed : Vec<i32> = Vec::new();

    loop {

        if instructions_passed.contains(&instruction_pointer) {
            break;
        }
        
        instructions_passed.push(instruction_pointer);

        let instruction : &Instruction = instructions.get(instruction_pointer as usize).unwrap();
        match instruction.command.as_str() {
            "acc" => {
                accumulator += instruction.value;
                instruction_pointer += 1;
            },
            "jmp" => {
                instruction_pointer += instruction.value;
            },
            "nop" => {
                instruction_pointer += 1;
            },
            _ => continue,
        }
    }

    println!("Accumulator value: {}", accumulator)



}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
