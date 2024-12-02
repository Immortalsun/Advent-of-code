use std::{fs::File, io::{BufRead, BufReader}, num::ParseIntError};

fn main() -> std::io::Result<()> {
   let f = File::open("src/input.txt")?;
   let mut reader = BufReader::new(f);
   let mut line = String::new();
   let mut len = reader.read_line(&mut line)?;
   let mut loc_ids_0: Vec<i64> = Vec::new();
   let mut loc_ids_1: Vec<i64> = Vec::new();

   //get inputs
   while len > 0 {
    let line_ints: Vec<i64> = get_ints(line.clone()).unwrap_or_else(|error| {
        panic!("Problem parsing ints from file: {error:?}");
    });
    loc_ids_0.push(line_ints[0]);
    loc_ids_1.push(line_ints[1]);
    line.clear();
    len = reader.read_line(&mut line)?;
   }

   //p@rt 1
   //sort inputs
   loc_ids_0.sort();
   loc_ids_1.sort();

   //accumulate distances
   let mut total_dist: i64 = 0;
   for idx in 0..loc_ids_0.len() {
    let dist: i64 = get_dist(loc_ids_0[idx], loc_ids_1[idx]);
    total_dist += dist;
   }

   println!("Total distance: {:?}", total_dist);

   //part 2
   let mut sim_score: i64 = 0;
   for idx in 0..loc_ids_0.len() {
    let sub_size: usize = loc_ids_1.iter().filter(|x| **x == loc_ids_0[idx]).count();
    let mut sub_score: i64 = i64::try_from(sub_size).unwrap_or_else(|error| {
        panic!("Problem converting size to int 64: {error:?}");
    });

    sub_score = loc_ids_0[idx] * sub_score;
    println!("Location {:?} occurs in right list {:?} times for a score of {:?}",loc_ids_0[idx], sub_size, sub_score);
    sim_score += sub_score;
   }

   println!("Total sim score is: {:?}", sim_score);
   Ok(())
}

fn get_ints(in_string: String) -> Result<Vec<i64>, ParseIntError> {
    let mut out_vec: Vec<i64> = Vec::new();
    let inputs: Vec<&str> = in_string.split_whitespace().collect();
    let value0: i64 = inputs[0].parse::<i64>()?;
    let value1: i64 = inputs[1].parse::<i64>()?;
    out_vec.push(value0);
    out_vec.push(value1);
    Ok(out_vec)
}

fn get_dist(first_loc: i64, sec_loc: i64) -> i64 {
    let dist: i64 = first_loc - sec_loc;
    dist.abs()
}