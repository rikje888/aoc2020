use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;



pub fn aoc15(){
    let mut arr: Vec<i64> = Vec::new();
    if let Ok(lines) = read_lines("aoc15input.txt") {
        for line in lines {
            if let Ok(l) = line {
                let t = l.split(",").collect::<Vec<&str>>();
                for i in t {
                    arr.push(i.parse::<i64>().unwrap());
                }
            }
        }
    }


    
    println!("Assignment 1: {}", assignment(&arr, 2020));
    println!("Assignment 2: {}", assignment(&arr, 30000000));
    
}


fn assignment(arr: &Vec<i64>, steps : i64) -> i64 {
    let mut numbers: HashMap<i64, (i64,i64)> = HashMap::new();
    let mut previous_number = 0;


    for i in 0..steps {
        let new_number;
        if i < arr.len() as i64{
            numbers.insert(arr[i as usize], (-1, i as i64));
            new_number = arr[i as usize];
        }
        else {
            let (a, b) = numbers.get(&previous_number).unwrap();
            if *a == -1 {
                //has never been called before
                new_number = 0;
            }
            else {
                new_number = *b - *a;
            }
            if !numbers.contains_key(&new_number){
                numbers.insert(new_number, (-1, -1));
            }
            let (_c, d) = numbers.get(&new_number).unwrap();
            numbers.insert(new_number, (*d, i as i64));

        }
        // if i % 1000 == 0 {
        //     println!("New number: {}", i);
        // }
        previous_number = new_number;
    }

    return previous_number;
}


fn assignment2(arr: &Vec<i64>) -> i64 {
    return 0;
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
