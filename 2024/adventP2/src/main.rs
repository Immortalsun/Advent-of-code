use std::{fs::File, io::{BufRead, BufReader}, num::ParseIntError};
use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    part2: String,
}
fn main() -> std::io::Result<()> {
    let f = File::open("src/input.txt")?;
    let args = Cli::parse();
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    let mut len = reader.read_line(&mut line)?;
    let mut safe_count: i64 = 0;
    let mut report_num = 1;
    //get inputs
    while len > 0 {
     let line_ints: Vec<i64> = get_ints(line.clone()).unwrap_or_else(|error| {
         panic!("Problem parsing ints from file: {error:?}");
     });

     //part 1
     //test line ints for safe report
     //before storage
     if is_safe(&line_ints) {
        safe_count +=1;
        println!("Report {:?}: {:?} is safe", report_num, line_ints);
     }
     //part 2
     else if args.part2 == "yes" &&  is_safe_dampened(&line_ints) {
        safe_count +=1;
        println!("Report {:?}: {:?} is safe by dampening", report_num, line_ints);
     }
     else {
        println!("Report {:?}:  {:?} is unsafe", report_num, line_ints);
     }
     report_num += 1;

     line.clear();
     len = reader.read_line(&mut line)?;
    }

    println!("The number of safe reports {:?}", safe_count);

    Ok(())
 }

 fn get_ints(in_string: String) -> Result<Vec<i64>, ParseIntError> {
     let mut out_vec: Vec<i64> = Vec::new();
     let inputs: Vec<&str> = in_string.split_whitespace().collect();

     for idx in 0..inputs.len() {
       let parsed_val: i64 = inputs[idx].parse::<i64>()?;
       out_vec.push(parsed_val);
     }

     Ok(out_vec)
 }

 fn is_safe(in_collection: &Vec<i64>) -> bool {
    //if input is sorted ascending or descending
    //we can continue
    if in_collection.is_sorted() || in_collection.is_sorted_by(|a, b| a >= b){
        //if we are sorted, then we need to check absolute value between idx and idx+1
        for idx in 0..in_collection.len() {
            if idx < in_collection.len() - 1 {
                let curr: i64 = in_collection[idx];
                let next: i64 = in_collection[idx + 1];
                let diff: i64 = curr - next;
                if diff.abs() < 1 || diff.abs() > 3{
                    return false;
                }
            }
        }
    }
    else {
       return false;
    }
    true
 }

 fn is_safe_dampened(in_collection: &Vec<i64>) -> bool {
    for idx in 0..in_collection.len() {
        let mut dampened_collection = in_collection.clone();
        dampened_collection.remove(idx);
        if is_safe(&dampened_collection) {
            return true;
        }
    }
   false
 }
