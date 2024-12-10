use std::{collections::HashSet, fs::File, io::{BufRead, BufReader}};
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
    let mut line_num: i32 = 0;
    let mut topographic_map: Vec<Vec<i32>> = Vec::new();
    let mut trailheads: HashSet<(i32, i32)> = HashSet::new();

    while len > 0 {
        let mut char_num: i32 = 0;
        let mut curr_collection: Vec<i32> = Vec::new();
        for c in line.trim_end().chars() {
            //parse number from character
            let num = c.to_string().parse::<i32>().unwrap();
            //if num is 0, record unique trailhead
            //line_num = row, char_num = col
            if num == 0 {
                trailheads.insert((line_num, char_num));
            }
            //add to collection
            curr_collection.push(num);
            char_num += 1;
        }
        //add to overall map
        topographic_map.push(curr_collection.clone());
        line.clear();
        len = reader.read_line(&mut line)?;
        line_num += 1;
    }

    if args.part2 != "yes" {
        //part 1
        //investigate each trailhead and find all of the 9's it can reach
        //a trailhead gets 1 point for each reachable 9
        let mut total_score: i32 = 0;
        for t in trailheads.iter() {
            let mut nines_coll: HashSet<(i32, i32)> = HashSet::new();
            let sub_score = find_unique_nines(0, t,
                &topographic_map, &mut nines_coll).len();
            total_score += sub_score as i32;
        }

        println!("Total score for map is: {:?}", total_score);
    }
    else {
        //part 2
        //investigate each trailhead for all the 9's it can reach, noting each time
        //it finds the same 9 is a unique path.
        //a trailhead gets 1 rating for each path, so the number of 9s in total should give
        //us that rating
        let mut total_rating: i32 = 0;
        for t in trailheads.iter() {
            let mut nines_coll: Vec<(i32, i32)> = Vec::new();
            let sub_rating = find_all_nines(0, t,
                &topographic_map, &mut nines_coll).len();
                total_rating += sub_rating as i32;
        }
        println!("Total rating for map is: {:?}", total_rating);
    }
    Ok(())
}

fn find_unique_nines<'a>(curr_num: i32, curr_pos: &(i32, i32), map: &'a Vec<Vec<i32>>, nines_found: &'a mut HashSet<(i32, i32)>)
 -> &'a HashSet<(i32, i32)> {
    if curr_num == 9 {
        nines_found.insert(*curr_pos);
        return nines_found;
    }

    else {
        //look in all 4 directions around cur_pos for curr_num+1
        let north = (curr_pos.0 - 1, curr_pos.1);
        let south= (curr_pos.0 + 1, curr_pos.1);
        let east = (curr_pos.0, curr_pos.1 + 1);
        let west = (curr_pos.0, curr_pos.1 - 1);

        if pos_is_valid(north, map) && pos_has_next_elevation(north, map, curr_num) {
            find_unique_nines(map[north.0 as usize][north.1 as usize], &north, map, nines_found);
        }

        if pos_is_valid(south, map) && pos_has_next_elevation(south, map, curr_num) {
            find_unique_nines(map[south.0 as usize][south.1 as usize], &south, map, nines_found);
        }

        if pos_is_valid(east, map) && pos_has_next_elevation(east, map, curr_num) {
            find_unique_nines(map[east.0 as usize][east.1 as usize], &east, map, nines_found);
        }

        if pos_is_valid(west, map) && pos_has_next_elevation(west, map, curr_num) {
            find_unique_nines(map[west.0 as usize][west.1 as usize], &west, map, nines_found);
        }
    }

    nines_found
}

fn find_all_nines<'a>(curr_num: i32, curr_pos: &(i32, i32), map: &'a Vec<Vec<i32>>, nines_found: &'a mut Vec<(i32, i32)>)
 -> &'a Vec<(i32, i32)> {
    if curr_num == 9 {
        nines_found.push(*curr_pos);
        return nines_found;
    }

    else {
        //look in all 4 directions around cur_pos for curr_num+1
        let north = (curr_pos.0 - 1, curr_pos.1);
        let south= (curr_pos.0 + 1, curr_pos.1);
        let east = (curr_pos.0, curr_pos.1 + 1);
        let west = (curr_pos.0, curr_pos.1 - 1);

        if pos_is_valid(north, map) && pos_has_next_elevation(north, map, curr_num) {
            find_all_nines(map[north.0 as usize][north.1 as usize], &north, map, nines_found);
        }

        if pos_is_valid(south, map) && pos_has_next_elevation(south, map, curr_num) {
            find_all_nines(map[south.0 as usize][south.1 as usize], &south, map, nines_found);
        }

        if pos_is_valid(east, map) && pos_has_next_elevation(east, map, curr_num) {
            find_all_nines(map[east.0 as usize][east.1 as usize], &east, map, nines_found);
        }

        if pos_is_valid(west, map) && pos_has_next_elevation(west, map, curr_num) {
            find_all_nines(map[west.0 as usize][west.1 as usize], &west, map, nines_found);
        }
    }

    nines_found
}

fn pos_is_valid(pos: (i32, i32), map: &Vec<Vec<i32>>) -> bool {
    return pos.0 >= 0 && pos.1 >= 0 && pos.0 < map.len() as i32
    && pos.1 < map[pos.0 as usize].len() as i32
}

fn pos_has_next_elevation(pos: (i32, i32), map: &Vec<Vec<i32>>, curr_elev: i32) -> bool {
    map[pos.0 as usize][pos.1 as usize] == curr_elev + 1
}