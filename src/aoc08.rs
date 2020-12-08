
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
use std::collections::HashSet;


struct Instruction {
    command: String,
    value: i32
}

impl Clone for Instruction {
    fn clone(&self) -> Instruction {
        let newInstruction = Instruction {
            command: self.command.to_string(),
            value: self.value,
        };
        newInstruction
    
    }
}

pub fn aoc08(){

    let mut original_instructions: Vec<Instruction> = Vec::new();
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
                original_instructions.push(i);


            }
        }
    }

    let mut accumulator = 0;

    let (finished, _accumulator, instructions_passed) = process_instructions(&original_instructions);
    accumulator = _accumulator;
    println!("Assignment 1: {}", accumulator);


    let mut skip = 0;

    loop {
        let mut skip_local = skip;
        let mut instructions = original_instructions.clone();
        for instruction_pointer in instructions_passed.iter().rev() {
            if skip_local >= 1 {
                skip_local -= 1;
                continue;
            }
            let instruction : &Instruction = instructions.get(*instruction_pointer as usize).unwrap();
            
            match instruction.command.as_str() {
                "jmp" => {
                    let replacement_instruction = Instruction {
                        command: "nop".to_string(),
                        value: instruction.value,
                    };

                    instructions.remove(*instruction_pointer as usize);
                    instructions.insert(*instruction_pointer as usize, replacement_instruction);

                    break;
                },
                "nop" => {
                    let replacement_instruction = Instruction {
                        command: "jmp".to_string(),
                        value: instruction.value,
                    };
                    
                    instructions.remove(*instruction_pointer as usize);
                    instructions.insert(*instruction_pointer as usize, replacement_instruction);
                    break;
                },
                "acc" => {
                    skip += 1;
                }
                _ => continue,
            }
        }

        
        let (finished, _accumulator, _ip) = process_instructions(&instructions);
        
        accumulator = _accumulator;
        if finished {
            break;
        }
        else {
            skip += 1;
        }

    }

    println!("Assignment 2: {}", accumulator);
    



}

fn process_instructions(instructions: &Vec<Instruction>) -> (bool, i32, Vec<i32>) {
    let mut accumulator = 0;
    let mut instruction_pointer :i32 = 0;

    let mut instructions_passed : Vec<i32> = Vec::new();

    loop {

        if instructions_passed.contains(&instruction_pointer) {
            return (false, accumulator, instructions_passed);
        }
        
        instructions_passed.push(instruction_pointer);
        if instruction_pointer >= instructions.len() as i32 {
            return (true, accumulator, instructions_passed);
        }

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

    // println!("Accumulator value: {}", accumulator)


}



fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
