use std::{fs::File, io::{BufRead, BufReader}};
use clap::Parser;
use regex::Regex;

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
    line = line.replace("\n", " ");
    //init regex
    let mult_extract = Regex::new(r"mul\((?<x>[0-9]{1,3}),(?<y>[0-9]{1,3})\)").unwrap();
    let enabled_grps = Regex::new(r"do\(\)(?<enabled>.+?)don't\(\)|do\(\)(?<endData>.+?)$").unwrap();
    let start_line_enabled_grps = Regex::new(r"^(?<enabled>.+?)don't\(\)").unwrap();
    let mut accumulated_mul: i64 = 0;
    //get inputs
    while len > 0 {
     len = reader.read_line(&mut line)?;
     line = line.replace("\n", " ");
    }

    //collect captures from each line
    if args.part2 != "yes" {
        let mul_caps: Vec<(&str, &str)> = mult_extract.captures_iter(line.as_str()).map(|caps| {
            let (_,[x_val, y_val]) = caps.extract();
             (x_val, y_val)
         }).collect();
 
         println!("mul instructions on line {:?}:", mul_caps);
 
      //part 1
      //iterate over each capture and do multiplication
      for idx in 0..mul_caps.len(){
         let x = mul_caps[idx].0.parse::<i64>().unwrap();
         let y = mul_caps[idx].1.parse::<i64>().unwrap();
         accumulated_mul += x * y;
      }
    }
   else {
       //part 2
       //for a given line
       //extract all string information before the first 'don't' starting from the beginning of the line
       let start_line_collection: Vec<&str> = start_line_enabled_grps.captures_iter(line.as_str()).map(|caps| {
        let (_, [start_line_data]) = caps.extract();
        start_line_data
       }).collect();
       println!("Start line data: {:?}", start_line_collection);
       //then extract all string information between 'do' and 'dont' commands
       let mut enabled_groups_collection: Vec<&str> = enabled_grps.captures_iter(line.as_str()).map(|caps| {
        let (_, [enabled_data]) = caps.extract();
        enabled_data
       }).collect();
       println!("enabled data: {:?}", enabled_groups_collection);
       //we should only have one value in start_line_collection, we can add it to enabled_groups
       //so that we only have to process one collection
       enabled_groups_collection.push(start_line_collection[0]);
       //now we need to extract all mults
       for idx in 0..enabled_groups_collection.len() {
        let line_mul_caps: Vec<(&str, &str)> = mult_extract.captures_iter(enabled_groups_collection[idx]).map(|caps| {
            let (_,[x_val, y_val]) = caps.extract();
             (x_val, y_val)
         }).collect();
         println!("Extracted values on enabled index {:?}: {:?}", idx, line_mul_caps);

         for idx in 0..line_mul_caps.len(){
            let x = line_mul_caps[idx].0.parse::<i64>().unwrap();
            let y = line_mul_caps[idx].1.parse::<i64>().unwrap();
            accumulated_mul += x * y;
         }
       }
   }

    println!("Accumulated results: {:?}", accumulated_mul);

    Ok(())
 }