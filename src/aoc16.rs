use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Range {
    start : i32,
    end : i32
}

pub fn aoc16(){
    let mut arr: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines("aoc16input.txt") {
        for line in lines {
            if let Ok(l) = line {
                arr.push(l);
            }
        }
    }

    let mut state = 0;
    let mut ranges : Vec<Vec<Range>> = Vec::new();
    let mut my_ticket : Vec<i32> = Vec::new();
    let mut other_tickets : Vec<Vec<i32>> = Vec::new();
    let mut error_rate = 0;
    for line in &arr{
        if line == "" {
            state += 1;
            continue;
        }
        if line.contains("ticket") {
            continue;
        }

        match state {
            0 => ranges.push(parse_range(line)),
            1 => my_ticket = parse_ticket(line),
            2 => other_tickets.push(parse_ticket(line)),
            _ => continue,
        }
    }

    other_tickets.retain(|ticket| {
        let this_error_rate = get_error_rate(ticket, &ranges);
        error_rate += this_error_rate;
        return this_error_rate == 0 
    });

    println!("Assignment 1: {}", error_rate);
    println!("Assignment 2: {}", assignment2(&other_tickets, &ranges, &my_ticket));
    
}

fn parse_ticket(line : &String) -> Vec<i32> {
    let mut ticket : Vec<i32> = Vec::new();
    let parts = line.split(",").collect::<Vec<&str>>();
    for part in parts {
        let value = part.parse::<i32>().unwrap();
        ticket.push(value);
    }
    return ticket;
}


fn parse_range(line : &String) -> Vec<Range>{
    let mut parts = line.split(":").collect::<Vec<&str>>()[1].split(" ").collect::<Vec<&str>>();
    
    let mut ranges : Vec<Range> = Vec::new();
    parts.remove(2);
    parts.remove(0);
    for part in parts {
        let range = Range {
            start: part.split("-").collect::<Vec<&str>>()[0].parse::<i32>().unwrap(),
            end: part.split("-").collect::<Vec<&str>>()[1].parse::<i32>().unwrap(),
        };
        ranges.push(range);
    }
    return ranges;
}

fn get_error_rate(ticket : &Vec<i32>, ranges : &Vec<Vec<Range>>) -> i32 {
    let mut error_rate = 0;
    for value in ticket {
        let mut valid = false;

        'outer: for outer in ranges {
            for inner in outer {
                if value >= &inner.start && value <= &inner.end {
                    valid = true;
                    break 'outer;
                }
            }
        }
        if !valid {
            error_rate += value;
        }
    }

    return error_rate;
}

fn assignment2(tickets : &Vec<Vec<i32>>, ranges : &Vec<Vec<Range>>, my_ticket : &Vec<i32>) -> i64 {
    let mut options : Vec<i32> = Vec::new();
    let mut indexes_assigned : Vec<i32> = Vec::new();
    let mut fields_assigned : Vec<i32> = Vec::new();
    for _i in 0..ranges.len(){
        options.push(-1);
    }

    let mut check_index = 0;
    loop {
        let mut fields_that_are_a_valid_option : Vec<i32> = Vec::new();
        for field in 0..ranges.len() {
            let mut is_candidate = true;
            'ticketsloop:
            for t in 0..tickets.len() {

                if (tickets[t][check_index] >= ranges[field][0].start && tickets[t][check_index] <= ranges[field][0].end) ||
                    (tickets[t][check_index] >= ranges[field][1].start && tickets[t][check_index] <= ranges[field][1].end) {
                    is_candidate = true;
                }
                else {
                    is_candidate = false;
                    break 'ticketsloop;
                }
            }
            if is_candidate {
                if !fields_assigned.contains(&(field as i32)){
                    fields_that_are_a_valid_option.push(field as i32);
                }
            }
        }
        if fields_that_are_a_valid_option.len() == 1 {
            options[fields_that_are_a_valid_option[0] as usize] = check_index as i32;
            indexes_assigned.push(check_index as i32);
            fields_assigned.push(fields_that_are_a_valid_option[0]);

            // println!("Assign field {} to index {}", fields_that_are_a_valid_option[0], check_index);

        }

        
        check_index = (check_index + 1) % ranges.len();
        while indexes_assigned.contains(&(check_index as i32)) {
            check_index = (check_index + 1) % ranges.len();
        }

        if indexes_assigned.len() == ranges.len() - 1  {
            break;
        }
    }


    let mut product :i64 = 1;

    for i in 0..6 {
        let mut index = 0;
        for j in 0..fields_assigned.len() {
            if fields_assigned[j] == i {
                index = j;
                break;
            }
        }
        // println!("My value {}", my_ticket[indexes_assigned[index] as usize]);
        product *= my_ticket[indexes_assigned[index] as usize] as i64;
    }




    return product;
}



fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
