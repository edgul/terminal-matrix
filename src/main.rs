use std::{thread, time};
use rand;
use rand::Rng;

use crossterm::{
    cursor,
    execute,
    terminal::{self, ClearType},
    ExecutableCommand,
};
use std::io::{stdout, Write};

mod matrix;
use matrix::Matrix;

fn random_number(n : usize) -> usize {
    rand::thread_rng().gen_range(0..n as u32) as usize
}

fn random_ascii() -> u8 {
    rand::thread_rng().gen_range(33..126)
}

fn main() {
    println!("wake up, neo");

    let (cols, rows) = terminal::size().unwrap();
    let mut matrix = Matrix::new(rows as usize, cols as usize);

    // feature flags
    let auto_quit_enabled = false;
    let blocks_enabled = true;
    let char_adding_enabled = true;
    let char_swapping_enabled = true;
    let sliding_viewport_enabled = true;
    let column_fade_enabled = true;

    let fade_height = 10;
    let frame_period = time::Duration::from_millis(3);
    let animation_length = time::Duration::from_millis(10000);

    let start = time::Instant::now();

    let mut stdout = stdout(); // Call the function to get the handle
    stdout.execute(terminal::Clear(ClearType::All)).unwrap();

    // paint loop
    loop {
        // add new char to the matrix
        let col = random_number(matrix.num_cols());
        if char_adding_enabled {
            let new_char = random_ascii() as char; 
            matrix.append_char_to_stream(col, new_char);
        }

        // swap chars
        if char_swapping_enabled {
            let swap_char = random_ascii() as char; 
            let swap_col = random_number(matrix.num_cols());
            let swap_col_height = matrix.column_height(swap_col);
            if swap_col_height > 0 {
                let swap_row = random_number(swap_col_height);
                matrix.overwrite_char(swap_row, swap_col, swap_char);
            }
        }  

        // divide into blocks
        if blocks_enabled {
            let block_size = matrix.num_cols() / 5;
            if block_size > 4 {
                let divisible_numbers: Vec<usize> = (0..matrix.num_cols()-1) // -1 to clear two columns
                    .filter(|&x| x % block_size == 0)
                    .collect();
                for i in divisible_numbers {
                    matrix.clear_col(i);
                    matrix.clear_col(i+1);
                }
            }
        }

        // column "fade"
        if column_fade_enabled {
            let h = matrix.column_height(col);
            if h > fade_height {
                matrix.overwrite_char(h - fade_height, col, matrix::BCHAR);
            }
        }

        // sliding viewport
        if sliding_viewport_enabled {
            if matrix.column_height(col) >= (matrix.num_rows() as u32).try_into().unwrap() {
                matrix.remove_row(0);
            }
        } 

        // paint it
        stdout.execute(cursor::MoveTo(0, 0)).unwrap();
        let mut count = 0;
        for row in matrix.rows() {
            print!("{}", row);
            stdout.execute(cursor::MoveTo(0, count)).unwrap();
            count += 1;
        }  

        thread::sleep(frame_period); // animation speed

        // auto-quit
        if auto_quit_enabled {
            if start.elapsed() >= animation_length {
                break;
            }
        }
    }
}
