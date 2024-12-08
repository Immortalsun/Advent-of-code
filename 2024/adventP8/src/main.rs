use std::{collections::HashSet, collections::HashMap, cmp, fs::File, io::{BufRead, BufReader}};
use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    part2: String
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Antenna {
    y: i64,
    x: i64
}


fn main() -> std::io::Result<()> {
    let f = File::open("src/input.txt")?;
    let args = Cli::parse();
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    let mut len = reader.read_line(&mut line)?;
    let mut signals_collection: HashMap<char, Vec<Antenna>> = HashMap::new();
    let mut line_num: i64 = 0;
    let mut max_col: i64 = 0;
    while len > 0 {
        let mut char_num: i64 = 0;
        for c in line.trim_end().chars() {
            match c {
                '.' => (),
                _ => {
                  let antennas=  signals_collection.entry(c).or_insert(Vec::new());
                  antennas.push(Antenna { y:line_num, x: char_num });
                }
            }
            char_num += 1;
        }
        line.clear();
        len = reader.read_line(&mut line)?;
        max_col = cmp::max(max_col, char_num);
        line_num += 1;
    }
    
    let max_row: i64 = line_num;
    //create set of antinode locations
    let mut antinode_locations: HashSet<(i64,i64)> = HashSet::new();

    if args.part2 != "yes" {
        //part 1
        //once we have built our collection of antenna signal keys to locations of those antennas
        //we need to calcultate the slope between two antennas, and find the points on the resulting line
        //corresponding to the antinodes

        //for each key (signal)
        for key in signals_collection.keys() {
            //get all antenna locations that correspond to that signal
            let antennas_with_signal = signals_collection[key].clone();
            //for all locations
            for i in 0..antennas_with_signal.len() {
                //chose one location
                let curr_antenna = antennas_with_signal[i];
                //compare with all other locations
                let other_antennas: Vec<&Antenna> = antennas_with_signal.iter()
                .filter(|x| **x != curr_antenna).collect();

                for j in 0..other_antennas.len() {
                    let test_antenna = other_antennas[j];
                    //find slope between current and test
                    let rise = test_antenna.y-curr_antenna.y;
                    let run = test_antenna.x-curr_antenna.x;

                    //apply positive and negative translation to current
                    let pos_translation = (curr_antenna.x + run, curr_antenna.y + rise);
                    let neg_translation = (curr_antenna.x - run, curr_antenna.y - rise);

                    //one result will be test, and the other will be an antinode
                    if pos_translation.0 == test_antenna.x && pos_translation.1 == test_antenna.y &&
                    is_valid_location(neg_translation.0, neg_translation.1, max_col, max_row) {
                        antinode_locations.insert(neg_translation);
                    }
                    else if neg_translation.0 == test_antenna.x && neg_translation.1 == test_antenna.y &&
                    is_valid_location(pos_translation.0, pos_translation.1, max_col, max_row) {
                        antinode_locations.insert(pos_translation);
                    }
                }
            }
        }

        println!("Unique antinode locations: {:?}", antinode_locations.len());
    }
    else {
        //part 2
        //for each key (signal)
        for key in signals_collection.keys() {
            //get all antenna locations that correspond to that signal
            let antennas_with_signal = signals_collection[key].clone();
            //for all locations
            for i in 0..antennas_with_signal.len() {
                //chose one location
                let curr_antenna = antennas_with_signal[i];
                //compare with all other locations
                let other_antennas: Vec<&Antenna> = antennas_with_signal.iter()
                .filter(|x| **x != curr_antenna).collect();

                for j in 0..other_antennas.len() {
                    let test_antenna = other_antennas[j];

                    //antennas are now all antinodes, so we should try
                    //to insert them if possible
                    antinode_locations.insert((curr_antenna.x, curr_antenna.y));
                    antinode_locations.insert((test_antenna.x, test_antenna.y));

                    //find slope between current and test
                    let rise = test_antenna.y-curr_antenna.y;
                    let run = test_antenna.x-curr_antenna.x;

                    //apply positive and negative translation to current
                    let mut pos_translation = (curr_antenna.x + run, curr_antenna.y + rise);
                    let mut neg_translation = (curr_antenna.x - run, curr_antenna.y - rise);

                    //keep moving along with each translation direction until we leave the map
                    while is_valid_location(pos_translation.0, pos_translation.1, max_col, max_row) {
                        antinode_locations.insert(pos_translation);
                        pos_translation = (pos_translation.0 + run, pos_translation.1 + rise);
                    }

                    while is_valid_location(neg_translation.0, neg_translation.1, max_col, max_row) {
                        antinode_locations.insert(neg_translation);
                        neg_translation = (neg_translation.0 - run, neg_translation.1 - rise);
                    }
                }
            }
        }

        println!("Unique antinode locations regardless of distance: {:?}", antinode_locations.len());
    }
    Ok(())
}

fn is_valid_location(x: i64, y: i64, max_x: i64, max_y: i64) -> bool {
    x >= 0 && x < max_x && y >= 0 && y < max_y
}