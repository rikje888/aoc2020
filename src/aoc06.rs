
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


pub fn aoc06(){

    let mut lines_array: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines("aoc06input.txt") {
        for line in lines {
            if let Ok(l) = line {
                //println!("{}", ip);
                lines_array.push(l);
            }
        }
    }

    let mut answers : String = "".to_string();
    let mut counters_cummulative = 0;
    let mut number_of_members_in_group = 0;
    for line in lines_array {
        match line.as_str() {
            "" => {
                counters_cummulative += count_answers(&answers, number_of_members_in_group);
                answers = "".to_string();
                number_of_members_in_group = 0;
            },
            _ => {
                number_of_members_in_group = number_of_members_in_group + 1;
                answers.push_str(line.as_str())
            },
        }
    }
    println!("Counters cummulative: {}", counters_cummulative)


}

fn count_answers(answers: &String, min : usize) -> i32{
    let mut counter = 0;
    for a in 'a'..='z'{
        if answers.matches(a).count() >= min {
            counter = counter + 1
        }
    }
    return counter;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
