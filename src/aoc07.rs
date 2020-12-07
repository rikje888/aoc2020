
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
use std::collections::HashSet;

struct Bag {
    children: HashMap<String, i32>,
    parents: Vec<String>
}


pub fn aoc07(){

    let mut lines_array: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines("aoc07input.txt") {
        for line in lines {
            if let Ok(l) = line {
                //println!("{}", ip);
                lines_array.push(l);
            }
        }
    }

    let mut bags : HashMap<String, Bag> = HashMap::new();

    for line in lines_array {
        let mut parts = line.split(" bags contain ").collect::<Vec<&str>>();
        let current_bag_name : String = parts[0].to_string();

        if !bags.contains_key(current_bag_name.as_str()){
            let mut bag = Bag { 
                children: HashMap::new(),
                parents: Vec::new()
            };
            bags.insert(current_bag_name.clone(), bag);
        }
        

       

        parts = parts[1].split(", ").collect::<Vec<&str>>();
        for part in parts {

            


            if !part.contains("no"){

                let mut name_part = part.split(" ").collect::<Vec<&str>>();
                name_part.remove(0);
                name_part.remove(name_part.len() - 1);
                let name_string = name_part.join(" ");
    
                if !bags.contains_key(name_string.as_str()){
                    let mut child_bag = Bag { 
                        children: HashMap::new(),
                        parents: Vec::new()
                    };
                    bags.insert(name_string.clone(), child_bag);
                }
                
                let child_bag = bags.get_mut(&name_string).unwrap();
                child_bag.parents.push(current_bag_name.clone());

                let integer_part = part.split(" ").collect::<Vec<&str>>()[0].parse::<i32>().unwrap();
                let bag = bags.get_mut(&current_bag_name).unwrap();
                bag.children.insert(name_string, integer_part);
            }
            

            

        }


        // bags.insert(current_bag_name, bag);

        println!("AAH");


    }

    let name = "shiny gold";
    let mut to_handle : Vec<String> = Vec::new();
    let mut handled : Vec<String> = Vec::new();
    to_handle.push(name.to_string());
    while to_handle.len() > 0 {
        let element = to_handle[0].clone();
        if !handled.contains(&element) {
            let bag = bags.get_mut(&element).unwrap();
            let mut iter = IntoIterator::into_iter(bag.parents.clone()); // strings is moved here
            for parent in iter {
                to_handle.push(parent);
            }
        }
        let mut i = 0;
        while i < to_handle.len() {
            if to_handle[i] == element {
                to_handle.remove(i);
            }
            else {
                i += 1;
            }
        }
        if !handled.contains(&element) {
            handled.push(element);
        }

    }

    println!("Size: {}", handled.len());

    println!("counter: {}", recursive_fuction(&bags, name.to_string()) - 1);


}

fn recursive_fuction(bags : &HashMap<String, Bag>, element: String) -> i32{
    let bag = bags.get(&element).unwrap();
    let mut counter = 1;
    for (child, nr) in bag.children.clone() {
        counter += nr * recursive_fuction(bags, child);
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
