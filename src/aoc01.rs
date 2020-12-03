use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

pub fn aoc01() {
    
    let mut arr = Vec::new();
    if let Ok(lines) = read_lines("aoc01input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                //println!("{}", ip);
                arr.push(ip.parse::<i32>().unwrap());
            }
        }
    }

    let start = Instant::now();

    arr.sort();
    let min = arr[0];

    //Assignment 1
    'outer1: for input_a in arr.iter() {
        for input_b in arr.iter() {
            if input_a + input_b > 2020 {
                break;
            }
            if input_a + input_b == 2020 {
                println!("Assignment 1: {}",
                    (input_a * input_b)
                );
                break 'outer1;
            }
        }
    }

    //Assignment 2
    'outer2: for input_a in arr.iter() {
        for input_b in arr.iter() {
            if input_a + input_b + min > 2020 {
                break;
            }
            for input_c in arr.iter() {
                if input_a + input_b + input_c > 2020 {
                    break;
                }
                if input_a + input_b + input_c == 2020 {
                    println!(
                        "Assignment 2: {}",
                        (input_a * input_b * input_c)
                    );
                    break 'outer2;
                }
            }
        }
    }

    let duration = start.elapsed();
    println!("Elapsed time: {:?}", duration)

}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
