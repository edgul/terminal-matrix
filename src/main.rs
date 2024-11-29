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

static CHAR_SWAP_FACTOR : usize = 5;

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
    let blocks_enabled = false;
    let char_swapping_enabled = false;
    let column_fade_enabled = true;

    let auto_quit_timeout = time::Duration::from_millis(10000);
    let start = time::Instant::now();

    let mut stdout = stdout(); // Call the function to get the handle
    stdout.execute(terminal::Clear(ClearType::All)).unwrap();

    // paint loop
    loop {
        let mut need_paint = false;

        // time-based character adding, shouldn't drift
        // though not sure how long this will run for safely
        let diff = time::Instant::now() - start;
        for ci in 0..matrix.num_cols() {
            let next = matrix.col_next_animation(ci);
            if diff > time::Duration::from_millis(next) {
                let new_char = random_ascii() as char; 
                matrix.append_char_to_column(ci, new_char);

                if column_fade_enabled {
                    let h = matrix.lead_index(ci);
                    let tail = h.checked_sub(matrix.tail_length(ci));
                    if let Some(tail_index) = tail {
                        if tail_index < matrix.num_rows() {
                            matrix.overwrite_char(tail.unwrap(), ci, matrix::BCHAR);
                        }
                    }
                }
                let animation_period = 10 * matrix.col_priority(ci) as u64;
                matrix.set_col_next_animation(ci, next + animation_period);
                need_paint = true;
            }
        }

        // swap chars
        if char_swapping_enabled {
            for _ in 0..CHAR_SWAP_FACTOR {
                let swap_char = random_ascii() as char; 
                let swap_col = random_number(matrix.num_cols());
                let col_lead = matrix.lead_index(swap_col);

                // -1 gives us a buffer so swapping doesn't result in stray chars
                let col_tail = col_lead.checked_sub(matrix.tail_length(swap_col) - 1);
                if let Some(col_tail) = col_tail {
                    let swap_row = rand::thread_rng().gen_range(col_tail as u32..col_lead as u32) as usize;
                    if swap_row < matrix.num_rows() && swap_row > 0 {
                        matrix.overwrite_char(swap_row, swap_col, swap_char);
                        need_paint = true;
                    }
                }
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

        // paint it only when needed
        if need_paint {
            stdout.execute(cursor::MoveTo(0, 0)).unwrap();
            let mut count = 0;
            for row in matrix.rows() {
                print!("{}", row);
                stdout.execute(cursor::MoveTo(0, count)).unwrap();
                count += 1;
            }  
        }

        // sleeping the thread for efficiency, not animation control 
        thread::sleep(time::Duration::from_millis(10));

        // auto-quit
        if auto_quit_enabled {
            if start.elapsed() >= auto_quit_timeout {
                break;
            }
        }
    }
}
