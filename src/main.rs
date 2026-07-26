use std::{thread, time};

use rand;
use rand::Rng;

use crossterm::{
    cursor, queue,
    style::{Print, SetForegroundColor},
    terminal::{self, ClearType},
    ExecutableCommand,
};
use std::io::{stdout, Write};

mod matrix;
use matrix::Matrix;

static CHAR_SWAP_FACTOR: usize = 5; // reduce to increase frequency of char swapping

fn random_number(n: usize) -> usize {
    rand::thread_rng().gen_range(0..n as u32) as usize
}

fn random_ascii() -> u16 {
    rand::thread_rng().gen_range(260..700)
}

fn main() {
    println!("wake up, neo");
    let (cols, rows) = terminal::size().unwrap();

    let mut matrix = Matrix::new(rows as usize, cols as usize);

    let auto_quit_enabled = false;
    let auto_quit_timeout = time::Duration::from_millis(10000);
    let start = time::Instant::now();
    let block_size: usize = (cols / 5) as usize;

    let blocks_enabled = true;
    let char_swapping_enabled = true;
    let mut last_swap = start;

    let mut stdout = stdout();
    stdout.execute(terminal::Clear(ClearType::All)).unwrap();

    loop {
        let diff = time::Instant::now() - start;

        for ci in 0..matrix.num_cols() {
            if blocks_enabled && ci % block_size == 0 {
                continue;
            }

            let next = matrix.col_next_animation(ci);
            if diff > time::Duration::from_millis(next) {
                let new_char = char::from_u32(random_ascii() as u32).unwrap();
                matrix.append_char_to_column(ci, new_char);

                let animation_period = 10 * matrix.col_priority(ci) as u64;
                matrix.set_col_next_animation(ci, next + animation_period);
            }
        }

        if char_swapping_enabled {
            let now = time::Instant::now();
            if now - last_swap > time::Duration::from_millis(CHAR_SWAP_FACTOR as u64) {
                let swap_char = char::from_u32(random_ascii() as u32);
                let swap_col = random_number(matrix.num_cols());
                let col_lead = matrix.lead_index(swap_col);
                let col_tail = col_lead.checked_sub(matrix.tail_length(swap_col));
                if let Some(col_tail) = col_tail {
                    let swap_row =
                        rand::thread_rng().gen_range(col_tail as u32..col_lead as u32) as usize;
                    if swap_row < matrix.num_rows() {
                        matrix.overwrite_char(swap_row, swap_col, swap_char.unwrap());
                    }
                }
                last_swap = now;
            }
        }

        let dirty_cells = std::mem::take(&mut matrix.dirty);
        if !dirty_cells.is_empty() {
            for cell in dirty_cells {
                queue!(
                    stdout,
                    cursor::MoveTo(cell.col as u16, cell.row as u16),
                    SetForegroundColor(cell.color),
                    Print(cell.character)
                )
                .unwrap();
            }
            stdout.flush().unwrap();
        }

        if auto_quit_enabled && start.elapsed() >= auto_quit_timeout {
            break;
        }

        thread::sleep(time::Duration::from_millis(1));
    }
}
