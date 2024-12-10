use std::{fs::File, io::{BufRead, BufReader}};
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
    let mut disk: Vec<i64> = Vec::new();
    let mut id_num: i64 = 0;
    let empty_space: i64 = -1;
    let visited_space: i64 = -2;
    while len > 0 {
        let mut char_num: i64 = 0;
        for c in line.trim_end().chars() {
            let num = c.to_string().parse::<i32>().unwrap();
            //a 0 means no data or no free space
            //either way we can skip it
            if num == 0 {
                char_num += 1;
                continue;
            }

            //every even indexed character is data
            if char_num % 2 == 0 {
                //place the id num a number of times = num
                for _ in 0..num {
                    disk.push(id_num);
                }
                //move to the next id number
                id_num += 1;
            }
            //odd idexed characters are free space
            else {
                //place empty space (represented by -1)
                 for _ in 0..num {
                    disk.push(empty_space);
                }
            }
            char_num += 1;
        }
        line.clear();
        len = reader.read_line(&mut line)?;
    }

    if args.part2 != "yes" {
        //start from the end of the disk and replace empty location
        let mut i: i32 = (disk.len() - 1).try_into().unwrap();
        while i >= 0 {
            let idx = i as usize;
            if disk[idx] > 0 {
                //find first available empty space
                let try_empty = disk.iter().position(|x| *x == empty_space);
                if try_empty.is_none() {
                    break;
                }
                let first_empty = try_empty.unwrap();
                //set the value of the empty space's position to the value we found at the index
                disk[first_empty] = disk[idx].clone();
                //clear the value at the index
                disk[idx] = visited_space;
            }
            else {
                 //clear the value at the index
                 disk[idx] = visited_space;
            }
            i-= 1;
        }
    }
    else {
        //the largest id num as found while building the disk
        //we subtract 1 to account for the loop ending after we increment one last time
        let max_id_num = id_num - 1;
        let mut cur_id_num: i64 = max_id_num;
        //examining all ids
        while cur_id_num >= 0 {
            //find elements with that id
            let id_collection: Vec<&i64> = disk.iter().filter(|x| **x == cur_id_num).collect();
            //get number of elements with that id
            let id_block_len = id_collection.len();
            //index in disk where id starts
            let id_start_idx = disk.iter().position(|x| *x == cur_id_num).unwrap();

            let mut skip_counter = 0;
            while skip_counter < id_start_idx {
                //find first available position of empty space
                //we may skip some if we cannot find enough space at the first position
                let first_empty = disk.iter().skip(skip_counter)
                .position(|x| *x == empty_space).unwrap();
                let mut empty_len: i64 = 0;
                //position will return elements offset from the start of the list
                //so if we skip any we need to adjust back to the original disk context
                //by adding the skip count to the first empty index
                let disk_search_start = first_empty + skip_counter;

                //if we found an empty position beyond the start of our data
                //its invalid, we don't need to continue searching
                if disk_search_start > id_start_idx {
                    break;
                }
                //starting at the next position after the empty slot, count contiguous empty slots
                //until we reach a non empty value
                for i in disk_search_start..disk.len() {
                   if disk[i] < 0 {
                        empty_len += 1;
                    }
                    else {
                        break;
                    }
                }

                if empty_len < id_block_len as i64 {
                    skip_counter = disk_search_start + empty_len as usize;
                }
                else {
                    let mut empty_idx: usize = disk_search_start;
                    for j in id_start_idx..id_start_idx+id_block_len {
                        disk[empty_idx] = disk[j].clone();
                        disk[j] = visited_space;
                        empty_idx += 1;
                    }
                    break;
                }
            }

            cur_id_num -= 1;
        }
    }

    //calculate checksum
    let mut check_sum: i64 = 0;
    for idx in 0..disk.len() {
        let int_idx = idx as i64;
        if disk[idx] < 0 {
            continue;
        }

        check_sum += int_idx * disk[idx];
    }

    println!("Checksum is {:?}", check_sum);
    Ok(())
}
