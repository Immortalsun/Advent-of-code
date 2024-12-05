use std::{cmp::Ordering, fs::File, io::{BufRead, BufReader}};
use std::collections::HashMap;
use std::collections::HashSet;

use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    part2: String,
}

fn main()  -> std::io::Result<()> {
    let f = File::open("src/input.txt")?;
    let args = Cli::parse();
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    let mut len = reader.read_line(&mut line)?;
    let mut order_rules: HashMap<i64, HashSet<i64>> = HashMap::new();
    let mut reading_page_lists = false;
    let mut p1_middle_page_num_agg: i64 = 0;
    let mut p2_middle_page_num_agg: i64 = 0;
    while len > 0 {

        if line == "\r\n" {
            reading_page_lists = true;
            line.clear();
            reader.read_line(&mut line)?;
        }

        if !reading_page_lists {
            //we are reading lines of the form XX|YY
            let trimmed_line = line.trim_end();
            let split_str:  Vec<&str> = trimmed_line.split_terminator('|').collect();
            let key: i64 = split_str[0].parse::<i64>().unwrap();
            let value: i64 = split_str[1].parse::<i64>().unwrap();
            let value_set = order_rules.entry(key).or_insert(HashSet::new());
            value_set.insert(value);
        }
        else {
            //collect page ordering lines
            let trimmed_line = line.trim_end();
            let split_str:  Vec<&str> = trimmed_line.split_terminator(',').collect();
            let mut page_ints: Vec<i64> = split_str.iter().map(|x| x.parse::<i64>().unwrap()).collect();

            if args.part2 != "yes" {
                //part 1 collects only those in the correct order
                if is_valid_page_list(&order_rules, &page_ints) {
                    p1_middle_page_num_agg += page_ints[page_ints.len()/2];
                }
            }
            else {
                //part 2 involves sorting the incorrectly ordered lines and aggregating the center
                if !is_valid_page_list(&order_rules, &page_ints) {
                    page_ints.sort_by(|a,b| {
                        if order_rules.contains_key(a) {
                            let value_set= order_rules.get(a).unwrap();
                            if value_set.contains(b) {
                                return Ordering::Less;
                            }
                            else {
                                Ordering::Greater
                            }
                        }
                        else {
                            Ordering::Greater
                        }
                    });

                    p2_middle_page_num_agg += page_ints[page_ints.len()/2];
                }
            }
        }
        line.clear();
        len = reader.read_line(&mut line)?;
       }

       if args.part2 != "yes" {
        println!("Part 1 middle page num aggregate is: {:?}", p1_middle_page_num_agg);
       }
       else {
           println!("Part 2 middle page num aggregate is:{:?}", p2_middle_page_num_agg);
       }
       Ok(())
}

fn is_valid_page_list(rule_set: &HashMap<i64, HashSet<i64>>, page_list: &Vec<i64>) -> bool {

    //we can skip the first element because there is
    //nothing to check preceding it
    for idx in 1..page_list.len() {
        let curr_num = page_list[idx];

        if rule_set.contains_key(&curr_num) {
            let value_set = rule_set.get(&curr_num).unwrap();
            //for a given curr_num key, the values in its associated hashSet
            //are values that it must occur before in the page list.
            //if we find any preceding values in the line that are in this hashset
            //we consider it a failure
            for sub_idx in 0..idx {
                if value_set.contains(&page_list[sub_idx]) {
                    return false;
                }
            }
        }
        else if idx != page_list.len() - 1 {
            return false;
        }
    }

    true
}
