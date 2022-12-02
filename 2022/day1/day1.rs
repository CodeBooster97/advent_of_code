use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // if the variable is immutable (default) not changable <3 
    let mut counter = 0; 
    let mut calories_list = Vec::new();
    
    if let Ok(lines) = read_lines("./input") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(calories) =  line {
                if calories != "" {
                    let calories = calories.parse::<i32>().unwrap();
                    counter = counter + calories;
                }
                else {
                    calories_list.push(counter);
                    counter = 0 ;
                }
            }
        } 
        let mut absolute = 0;
        for i in 0..3{
            let max_value = *calories_list.iter().max().unwrap();
            calories_list.remove(calories_list.iter().position(|x| *x == max_value).expect("max not found"));
            absolute = absolute + max_value;
            println!("{absolute} is the max value in loop {i}")
        }
    }
}

// read file input by line 
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}