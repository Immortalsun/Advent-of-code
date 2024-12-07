use std::{collections::HashSet, fs::File, io::{BufRead, BufReader}};
use std::hash::{Hash, Hasher};
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
            Direction::W => Some(Direction::N),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct GuardPosition {
    row: i64,
    col: i64,
    dir: Direction
}

impl Hash for GuardPosition {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.row.hash(state);
        self.col.hash(state);
        self.dir.hash(state);
    }
}

impl PartialEq for GuardPosition {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row
        && self.col == other.col
        && self.dir == other.dir
    }
}

impl Eq for GuardPosition {}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct PartialGuardPosition {
    row: i64,
    col: i64
}

fn main()  -> std::io::Result<()> {
    let f = File::open("src/input.txt")?;
    let args = Cli::parse();
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    let mut len = reader.read_line(&mut line)?;
    let mut start_point: (i64, i64) = (0,0);
    let mut lab_map: Vec<Vec<i64>> = Vec::new();
    let mut line_num: i64 = 0;

    while len > 0 {
        let mut current_line_collection: Vec<i64> = Vec::new();
        let mut char_num: i64 = 0;
        for c in line.chars() {
            match c {
                '.' => current_line_collection.push(0),
                '#' => current_line_collection.push(1),
                '^' => {
                        current_line_collection.push(2);
                        start_point = (line_num, char_num);
                    },
                _ => ()
            }
            char_num += 1;
        }
        lab_map.push(current_line_collection.clone());
        line.clear();
        len = reader.read_line(&mut line)?;
        line_num += 1;
    }

    if args.part2 != "yes" {
        println!("Distinct visit count: {:?}", plot_path_simple(&start_point, &lab_map).len());
    }
    else {
        //part 2
        //find predicated path from part 1
        let predicted_path = plot_path_simple(&start_point, &lab_map);
        //for each position determine if it produces a loop
        let obstacles: Vec<&PartialGuardPosition> = predicted_path.iter().filter(|x| plot_path_find_loops(&start_point, &lab_map, &(x.row, x.col))).collect();
        println!("Numbr of obstacle positions: {:?}", obstacles.len());
    }
    Ok(())
}

fn position_is_valid(curr_pos: &(i64, i64), lab_map: &Vec<Vec<i64>>) -> bool {
    if curr_pos.0 >= 0 && curr_pos.1 >= 0 &&
    curr_pos.0 < lab_map.len() as i64 && curr_pos.1 < lab_map[0].len() as i64 {
      return  true;
    }

    false
}

fn plot_path_simple(start_pos:&(i64, i64), lab_map: &Vec<Vec<i64>>) -> HashSet<PartialGuardPosition> {
    let mut predicted_path: HashSet<PartialGuardPosition> = HashSet::new();
    let obstacle = 1;
    predicted_path.insert(PartialGuardPosition { row:start_pos.0, col: start_pos.1 });
    let mut curr_pos = start_pos.clone();
    let mut curr_dir: Direction = Direction::N;
    let mut leaving_map = false;
    while !leaving_map && position_is_valid(&curr_pos, &lab_map) {
        let mut saw_obstacle = false;
        let mut new_row = curr_pos.0;
        let mut new_col = curr_pos.1;
        match curr_dir {
            Direction::N => {
                leaving_map = curr_pos.0 - 1 < 0;
                if !leaving_map {
                    new_row = curr_pos. 0 - 1;
                    saw_obstacle = lab_map[new_row as usize][curr_pos.1 as usize] == obstacle;
                }
            },
            Direction::S => {
                leaving_map = curr_pos.0 + 1 >= lab_map.len() as i64;
                if !leaving_map  {
                    new_row = curr_pos. 0 + 1;
                    saw_obstacle = lab_map[new_row as usize][curr_pos.1 as usize] == obstacle;
                }
            },
            Direction::E => {
                leaving_map = curr_pos.1 + 1 >= lab_map[curr_pos.0 as usize].len() as i64;
                if !leaving_map {
                    new_col = curr_pos.1 + 1;
                    saw_obstacle = lab_map[curr_pos.0 as usize][new_col as usize] == obstacle;
                }
            },
            Direction::W => {
                leaving_map = curr_pos.1 - 1 < 0;
                if !leaving_map  {
                    new_col = curr_pos.1 - 1;
                    saw_obstacle = lab_map[curr_pos.0 as usize][new_col as usize] == obstacle;
                }
            }
        }

        if leaving_map {
            continue;
        }

        if saw_obstacle {
            curr_dir = curr_dir.into_iter().next().unwrap();
        }
        else {
            curr_pos = (new_row, new_col);
            let curr_guard_pos = PartialGuardPosition { row: new_row,
                col: new_col};
                predicted_path.insert(curr_guard_pos);
        }
    }

    predicted_path

}

fn plot_path_find_loops(start_pos:&(i64, i64), lab_map: &Vec<Vec<i64>>,
    new_obstacle_pos:&(i64, i64)) -> bool {

        let mut predicted_path: HashSet<GuardPosition> = HashSet::new();
        let obstacle = 1;
        let mut added_obstacle_map = lab_map.clone();
        added_obstacle_map[new_obstacle_pos.0 as usize][new_obstacle_pos.1 as usize] = obstacle;
        predicted_path.insert(GuardPosition { row:start_pos.0, col: start_pos.1 , dir:Direction::N});
        let mut curr_pos = start_pos.clone();
        let mut curr_dir: Direction = Direction::N;
        let mut leaving_map = false;
        while !leaving_map && position_is_valid(&curr_pos, &added_obstacle_map) {
            let mut saw_obstacle = false;
            let mut new_row = curr_pos.0;
            let mut new_col = curr_pos.1;
            match curr_dir {
                Direction::N => {
                    leaving_map = curr_pos.0 - 1 < 0;
                    if !leaving_map {
                        new_row = curr_pos. 0 - 1;
                        saw_obstacle = added_obstacle_map[new_row as usize][curr_pos.1 as usize] == obstacle;
                    }
                },
                Direction::S => {
                    leaving_map = curr_pos.0 + 1 >= lab_map.len() as i64;
                    if !leaving_map  {
                        new_row = curr_pos. 0 + 1;
                        saw_obstacle = added_obstacle_map[new_row as usize][curr_pos.1 as usize] == obstacle;
                    }
                },
                Direction::E => {
                    leaving_map = curr_pos.1 + 1 >= lab_map[curr_pos.0 as usize].len() as i64;
                    if !leaving_map {
                        new_col = curr_pos.1 + 1;
                        saw_obstacle = added_obstacle_map[curr_pos.0 as usize][new_col as usize] == obstacle;
                    }
                },
                Direction::W => {
                    leaving_map = curr_pos.1 - 1 < 0;
                    if !leaving_map  {
                        new_col = curr_pos.1 - 1;
                        saw_obstacle = added_obstacle_map[curr_pos.0 as usize][new_col as usize] == obstacle;
                    }
                }
            }

            if leaving_map {
                continue;
            }

            if saw_obstacle {
                curr_dir = curr_dir.into_iter().next().unwrap();
            }
            else {
                curr_pos = (new_row, new_col);
                let curr_guard_pos = GuardPosition { row: new_row,
                    col: new_col, dir: curr_dir.clone()};

                   if !predicted_path.insert(curr_guard_pos) {
                    //if we have seen the same position while traveling in the same
                    //direction, we are in a loop
                    println!("Loop found using {:?}",new_obstacle_pos);
                    return true;
                   }
            }
        }

        //if we left the map, we did not find a loop
        false
    
}