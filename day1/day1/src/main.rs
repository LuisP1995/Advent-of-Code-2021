use std::env;
use std::fs::File;
use std::io::{BufRead,BufReader};

/**
* Main will attempt to solve day 1 of advent of code
**/ 
fn main() {
    let args: Vec<String> = env::args().collect();
    // Get the input from the command line arguments
    let filename = &args[1];
    let mut counter = 0;
    let mut data = Vec::new();
    println!("Opening a file");
    load_data(&filename, &mut data);
    let solution = solve_day1(&mut data, &mut counter);
    println!("increases found were {}", solution);
} 

fn load_data(filename: &String,container :&mut Vec<i32>){

    println!("reading a file {}", filename);

    // A buffered reader to run all the 
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    for (_index, line) in reader.lines().enumerate(){
        let line = line.unwrap();
        container.push(line.parse::<i32>().unwrap());
        //println!("current data in line {}", line);
    }
}

fn solve_day1(values: &mut Vec<i32>, counter: &mut i32) -> i32{
    let mut previous_value = values[0];
    for &current_value in values.iter() {
        if current_value == previous_value{
            continue;
            // *counter +=1;
            // previous_value = current_value;
        }else if current_value > previous_value{
            *counter +=1;

        }
        previous_value = current_value;
        println!("{}", previous_value);
    }
    return *counter;
}