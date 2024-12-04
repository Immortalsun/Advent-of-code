use std::{fs::File, io::{BufRead, BufReader}};
use clap::Parser;

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
    let mut input_collection: Vec<Vec<char>> = Vec::new();
    let x_char = 'X';
    let a_char = 'A';

    while len > 0 {
        let mut current_line_collection: Vec<char> = Vec::new();
        for c in line.chars() {

            if c != '\r' && c != '\n'{
                current_line_collection.push(c);
            }

        }
        input_collection.push(current_line_collection.clone());
        line.clear();
        len = reader.read_line(&mut line)?;
       }

       if args.part2 != "yes" {
        let mut xmas_count = 0;
        //part 1
        //we need to find XMAS in any order
        //so we look for X and then from that index check each position for MAS
            //cardinal directions:
            // up to 3 indices to the right
            // up to 3 indices down
            // up to -3 indices to the left
            // up to -3 indices up
            //intermediate directions:
            //3 right, 3 down
            //3 right, -3 up
            //-3 left, 3 down
            //-3 left, -3 up
            for row in 0..input_collection.len(){
                for col in 0..input_collection[row].len() {
                    let curr_char = input_collection[row][col];

                    //if we see an X
                    if curr_char == x_char {
                        //check for MAS in all directions
                        if check_mas_north(&input_collection, row, col) {
                            xmas_count+= 1;
                        }

                        if check_mas_south(&input_collection, row, col) {
                            xmas_count+= 1;
                        }

                        if check_mas_east(&input_collection, row, col) {
                            xmas_count+= 1;
                        }

                        if check_mas_west(&input_collection, row, col) {
                            xmas_count+= 1;
                        }

                        if check_mas_northeast(&input_collection, row, col) {
                            xmas_count+= 1;
                        }

                        if check_mas_southeast(&input_collection, row, col) {
                            xmas_count+= 1;
                        }

                        if check_mas_northwest(&input_collection, row, col) {
                            xmas_count+= 1;
                        }

                        if check_mas_southwest(&input_collection, row, col) {
                            xmas_count+= 1;
                        }
                    }
                }
            }

            println!("Xmas count is: {:?}", xmas_count);
       }
       else {
        //part 2
        let mut x_mas_count = 0;
        for row in 0..input_collection.len(){
            for col in 0..input_collection[row].len() {
                let curr_char = input_collection[row][col];

                //if we see an A
                if curr_char == a_char {
                    //check for MS in all diagonal directions

                    if check_ms_east_diag(&input_collection, row, col)
                    && check_ms_west_diag(&input_collection, row, col) {
                        x_mas_count+= 1;
                    }
                }
            }
        }
        println!("X-mas count is: {:?}", x_mas_count);
    }

    Ok(())
}



fn check_mas_east(input_coll: &Vec<Vec<char>>, start_row: usize, start_col: usize) -> bool {

    //if we can't move east at least 3 spaces, return false
    if start_col + 3 > input_coll[start_row].len() - 1 {
        return false;
    }

    if input_coll[start_row][start_col + 1] == 'M'
    && input_coll[start_row][start_col + 2] == 'A'
    && input_coll[start_row][start_col + 3] == 'S'{
        return true;
    }

    false
}

fn check_mas_west(input_coll: &Vec<Vec<char>>, start_row: usize, start_col: usize) -> bool {

     //if we can't move west at least 3 spaces, return false
     let col_int: i32 = start_col.try_into().unwrap();
     if col_int - 3 < 0 {
        return false;
    }

    if input_coll[start_row][start_col - 1] == 'M'
    && input_coll[start_row][start_col - 2] == 'A'
    && input_coll[start_row][start_col - 3] == 'S'{
        return true;
    }


    false
}

fn check_mas_north(input_coll: &Vec<Vec<char>>, start_row: usize, start_col: usize) -> bool {

    //if we can't move north at least 3 spaces, return false
    let row_int: i32 = start_row.try_into().unwrap();
    if row_int - 3 < 0 {
       return false;
   }

   if input_coll[start_row-1][start_col] == 'M'
   && input_coll[start_row-2][start_col] == 'A'
   && input_coll[start_row-3][start_col] == 'S'{
       return true;
   }
    false
}

fn check_mas_south(input_coll: &Vec<Vec<char>>, start_row: usize, start_col: usize) -> bool {

    if start_row + 3 > input_coll.len() - 1 {
       return false;
   }

   if input_coll[start_row+1][start_col] == 'M'
   && input_coll[start_row+2][start_col] == 'A'
   && input_coll[start_row+3][start_col] == 'S'{
       return true;
   }
    false
}


fn check_mas_northeast(input_coll: &Vec<Vec<char>>, start_row: usize, start_col: usize) -> bool {

    let row_int: i32 = start_row.try_into().unwrap();
    if start_col + 3 > input_coll[start_row].len() - 1 || row_int - 3 < 0 {
        return false;
    }

    if input_coll[start_row-1][start_col+1] == 'M'
    && input_coll[start_row-2][start_col+2] == 'A'
    && input_coll[start_row-3][start_col+3] == 'S'{
        return true;
    }
    false
}

fn check_mas_northwest(input_coll: &Vec<Vec<char>>, start_row: usize, start_col: usize) -> bool {

    let row_int: i32 = start_row.try_into().unwrap();
    let col_int: i32 = start_col.try_into().unwrap();
    if col_int - 3 < 0 || row_int - 3 < 0 {
        return false;
    }

    if input_coll[start_row-1][start_col-1] == 'M'
    && input_coll[start_row-2][start_col-2] == 'A'
    && input_coll[start_row-3][start_col-3] == 'S'{
        return true;
    }
    false
}

fn check_mas_southeast(input_coll: &Vec<Vec<char>>, start_row: usize, start_col: usize) -> bool {

    if start_row + 3 > input_coll.len() - 1 || start_col + 3 > input_coll[start_row].len() - 1{
        return false;
    }

    if input_coll[start_row+1][start_col+1] == 'M'
    && input_coll[start_row+2][start_col+2] == 'A'
    && input_coll[start_row+3][start_col+3] == 'S'{
        return true;
    }
    false
}

fn check_mas_southwest(input_coll: &Vec<Vec<char>>, start_row: usize, start_col: usize) -> bool {
    let col_int: i32 = start_col.try_into().unwrap();
    if start_row + 3 > input_coll.len() - 1 || col_int - 3 < 0 {
        return false;
    }

    if input_coll[start_row+1][start_col-1] == 'M'
    && input_coll[start_row+2][start_col-2] == 'A'
    && input_coll[start_row+3][start_col-3] == 'S'{
        return true;
    }
    false
}


fn check_ms_east_diag(input_coll: &Vec<Vec<char>>, start_row: usize, start_col: usize) -> bool {

    let row_int: i32 = start_row.try_into().unwrap();
    let col_int: i32 = start_col.try_into().unwrap();
    if start_col + 1 > input_coll[start_row].len() - 1 || row_int - 1 < 0 ||
    col_int - 1 < 0 || start_row + 1 > input_coll.len() - 1 {
        return false;
    }

    if (input_coll[start_row-1][start_col+1] == 'M' && input_coll[start_row+1][start_col-1] == 'S')
    || (input_coll[start_row-1][start_col+1] == 'S' && input_coll[start_row+1][start_col-1] == 'M'){
        return true;
    }

    false
}

fn check_ms_west_diag(input_coll: &Vec<Vec<char>>, start_row: usize, start_col: usize) -> bool {

    let row_int: i32 = start_row.try_into().unwrap();
    let col_int: i32 = start_col.try_into().unwrap();
    if start_col + 1 > input_coll[start_row].len() - 1 || row_int - 1 < 0 ||
    col_int - 1 < 0 || start_row + 1 > input_coll.len() - 1 {
        return false;
    }

    if (input_coll[start_row-1][start_col-1] == 'M' && input_coll[start_row+1][start_col+1] == 'S')
    || (input_coll[start_row-1][start_col-1] == 'S' && input_coll[start_row+1][start_col+1] == 'M'){
        return true;
    }

    false
}
