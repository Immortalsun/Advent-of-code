use std::{ collections::HashMap, fs::File, io::{BufRead, BufReader}};
use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    part2: String
}

fn main() -> std::io::Result<()> {
    let f = File::open("src/input.txt")?;
    let args = Cli::parse();
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    let mut len = reader.read_line(&mut line)?;
    let mut stone_collection: Vec<i64> = Vec::new();

    while len > 0 {
        let split_coll: Vec<&str> = line.trim_end().split_whitespace().collect();

        for n in split_coll.iter() {
            stone_collection.push(n.parse::<i64>().unwrap());
        }

        line.clear();
        len = reader.read_line(&mut line)?;
    }

    if args.part2 != "yes" {
        //part 1
        for _i in 0..25 {
            stone_collection = blink_p1(&stone_collection);
        }

        println!("After 25 blinks there are {:?} stones", stone_collection.len());
    }
    else {
        //part 2
        let stones_count: i64 = blink_p2(&stone_collection, 75);
        println!("After 75 blinks there are {:?} stones", stones_count);
    }
    Ok(())
}

fn blink_p1(stone_collection: &Vec<i64>) -> Vec<i64> {
    //for part 1 there are 3 cases
    //if the stone is 0, it becomes 1
    //if the stone has an even number of digits, it is replaced by two stone, one with each half of num
    //otherwise the stone's value is multiplied by 2024
    let mut output: Vec<i64> = Vec::new();

    for i in 0..stone_collection.len() {
        let stone = stone_collection[i];
        if stone == 0 {
            output.push(1);
        }
        else if stone.to_string().len() % 2 == 0 {
            let stone_str = stone.to_string();
            let stone_pair = stone_str.split_at(stone_str.len()/2);
            let left_stone = stone_pair.0.parse::<i64>().unwrap();
            let right_stone = stone_pair.1.parse::<i64>().unwrap();
            output.push(left_stone);
            output.push(right_stone);
        }
        else {
            output.push(stone * 2024);
        }
    }

    output
}

fn blink_p2(stone_collection: &Vec<i64>, times_to_blink: i64)
 -> i64 {
    let mut count: i64 = 0;
    let mut mem_cache: HashMap<(i64,i64), i64> = HashMap::new();
    for stone in stone_collection {
        count += blink_and_save_state(*stone, times_to_blink,
        &mut mem_cache);
    }
    count
}

fn blink_and_save_state(stone_val: i64, recurrence_counter: i64, mem_cache: &mut HashMap<(i64, i64), i64>) -> i64 {

    if recurrence_counter == 0 {
        return 1;
    }

    if mem_cache.contains_key(&(stone_val, recurrence_counter)){
        return *mem_cache.get(&(stone_val, recurrence_counter)).unwrap();
    }

    let result_count: i64;

    if stone_val == 0 {
        result_count =
            blink_and_save_state(1, recurrence_counter-1, mem_cache);
    }
    else if stone_val.to_string().len() % 2 == 0 {
        let stone_str = stone_val.to_string();
        let stone_pair = stone_str.split_at(stone_str.len()/2);
        let left_stone = stone_pair.0.parse::<i64>().unwrap();
        let right_stone = stone_pair.1.parse::<i64>().unwrap();
        result_count = blink_and_save_state(left_stone, recurrence_counter-1, mem_cache) +
            blink_and_save_state(right_stone, recurrence_counter-1, mem_cache);
    }
    else {
       result_count = blink_and_save_state(stone_val * 2024, recurrence_counter-1, mem_cache);
    }

    mem_cache.insert((stone_val, recurrence_counter), result_count);
    result_count
}