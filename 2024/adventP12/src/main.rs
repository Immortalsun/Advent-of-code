use std::{collections::HashSet, fs::File, io::{BufRead, BufReader}};
use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    part2: String
}

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
enum Direction {
    N,
    S,
    E,
    W,
}

impl Iterator for Direction {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Direction::N => Some(Direction::E),
            Direction::S => Some(Direction::W),
            Direction::E => Some(Direction::S),
            Direction::W => None,
        }
    }
}

const DIRECTIONS: &'static [&'static Direction] = &[&Direction::N, &Direction::S, &Direction::E, &Direction::W];

fn main() -> std::io::Result<()> {
    let f = File::open("src/input.txt")?;
    let args = Cli::parse();
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    let mut len = reader.read_line(&mut line)?;
    let mut plot_map: Vec<Vec<char>> = Vec::new();

    while len > 0 {
        let mut curr_collec: Vec<char> = Vec::new();
        for c in line.trim_end().chars() {
            curr_collec.push(c);
        }
        plot_map.push(curr_collec.clone());
        line.clear();
        len = reader.read_line(&mut line)?;
    }

    if args.part2 != "yes" {
        //part 1
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let mut total: i64 = 0;
        for row in 0..plot_map.len() {
            let curr_row = &plot_map[row];
            for col in 0..curr_row.len() {
                if visited.contains(&(row, col)) {
                    continue;
                }
                else {
                    let mut perimeter: i64 = 0;
                    let mut area: i64 = 0;
                    search_region((row, col), plot_map[row][col],
                     &mut visited, &plot_map, &mut perimeter, &mut area);
                    println!("Region with designation {:?} has area: {:?}, perimeter: {:?}", plot_map[row][col], area ,perimeter);
                    total += area * perimeter;
                }
            }
        }
        println!("Total price for all plots: {:?}", total);
    }
    else {
        //part 2
    }
    Ok(())
}

fn search_region(start_point:(usize, usize), designation: char,
    visited: &mut HashSet<(usize, usize)>, plot_map: &Vec<Vec<char>>, perimeter: &mut i64,
    area: &mut i64) {
        visited.insert(start_point.clone());
        *area += 1;
        for d in DIRECTIONS {
            let next_pos = translate_point(&(start_point.0 as i64, start_point.1 as i64),
             **d);
            if !position_is_valid(&next_pos, plot_map) {
                *perimeter += 1;
                continue;
            }
            let next_point = (next_pos.0 as usize, next_pos.1 as usize);
            if plot_map[next_point.0][next_point.1] == designation {
                if !visited.contains(&next_point) {
                    search_region(next_point, designation, visited, plot_map, perimeter, area);
                }
            }
            else {
                *perimeter += 1;
            }
        }
}

fn position_is_valid(curr_pos: &(i64, i64), map: &Vec<Vec<char>>) -> bool {
    if curr_pos.0 >= 0 && curr_pos.1 >= 0 &&
    curr_pos.0 < map.len() as i64 && curr_pos.1 < map[0].len() as i64 {
      return  true;
    }

    false
}

fn translate_point(curr_pos: &(i64, i64), dir: Direction) -> (i64, i64) {

    match dir {
        Direction::N => (curr_pos.0 - 1,curr_pos.1),
        Direction::S => (curr_pos.0 + 1, curr_pos.1),
        Direction::E => (curr_pos.0, curr_pos.1 + 1),
        Direction::W => (curr_pos.0, curr_pos.1 - 1),
    }
}

