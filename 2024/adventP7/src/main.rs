use std::{ fs::File, io::{BufRead, BufReader}};
use clap::Parser;
use itertools::Itertools;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    part2: String
}

#[derive(Debug, EnumIter, Clone, Copy)]
enum Operator {
    ADD,
    MULT,
}

#[derive(Debug, EnumIter, Clone, Copy)]
enum ExpandedOperator {
    ADD,
    MULT,
    CONCAT
}

fn main()  -> std::io::Result<()>  {
    let f = File::open("src/input.txt")?;
    let args = Cli::parse();
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    let mut len = reader.read_line(&mut line)?;
    let mut valid_accumulator: u64 = 0;

    while len > 0 {
        let processed_values = process_line(&line);
        if args.part2 != "yes" {
            if line_produces_valid_result(&processed_values.0, &processed_values.1) {
                valid_accumulator += processed_values.0;
            }
        }
        else {
            if line_produces_valid_result_expanded(&processed_values.0, &processed_values.1) {
                valid_accumulator += processed_values.0;
            }
        }

        line.clear();
        len = reader.read_line(&mut line)?;
    }

    println!("Accumlated valid results: {:?}", valid_accumulator);
    Ok(())
}

fn process_line(input_line: &String) -> (u64, Vec<u64>) {
    let split_val: Vec<&str> = input_line.split_terminator(":").collect();
    let target_val: u64 = split_val[0].parse::<u64>().unwrap();
    let split_sub_values: Vec<&str> = split_val[1].split_whitespace().collect();
    let mut sub_values: Vec<u64> = Vec::new();
    for idx in 0..split_sub_values.len() {
        sub_values.push(split_sub_values[idx].parse::<u64>().unwrap());
    }

    (target_val, sub_values)
}

fn line_produces_valid_result(target_val: &u64, input_vals: &Vec<u64>) -> bool {
    //calculate number of operators we need
    let num_operators = input_vals.len() - 1;
    //use multi cartesian product to produce collections of operators (with duplicates allowed)
    //up to the number that we need
    for op_set in (0..num_operators).map(|_| Operator::iter()).multi_cartesian_product() {
        let mut accumulated_val = input_vals[0];
        for idx in 1..input_vals.len() {
            let operator = op_set[idx-1];
            match operator {
                Operator::ADD => accumulated_val = accumulated_val + input_vals[idx],
                Operator::MULT => accumulated_val = accumulated_val * input_vals[idx],
            }
        }

        if accumulated_val == *target_val {
            println!("Valid operator set: {:?} on {:?} gives {:?}", op_set, input_vals, target_val);
            return true;
        }
    }
    false
}

fn line_produces_valid_result_expanded(target_val: &u64, input_vals: &Vec<u64>) -> bool {
    //calculate number of operators we need
    let num_operators = input_vals.len() - 1;
    //use multi cartesian product to produce collections of operators (with duplicates allowed)
    //up to the number that we need
    for op_set in (0..num_operators).map(|_| ExpandedOperator::iter()).multi_cartesian_product() {
        let mut accumulated_val = input_vals[0];
        for idx in 1..input_vals.len() {
            let operator = op_set[idx-1];
            match operator {
                ExpandedOperator::ADD => accumulated_val = accumulated_val + input_vals[idx],
                ExpandedOperator::MULT => accumulated_val = accumulated_val * input_vals[idx],
                ExpandedOperator::CONCAT => {
                    let mut accum_val_string = accumulated_val.to_string();
                    accum_val_string.push_str(&input_vals[idx].to_string());
                    accumulated_val = accum_val_string.parse::<u64>().unwrap();
                },
            }
        }

        if accumulated_val == *target_val {
            println!("Valid operator set: {:?} on {:?} gives {:?}", op_set, input_vals, target_val);
            return true;
        }
    }
    false
}