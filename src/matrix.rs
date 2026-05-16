
use rand;
use rand::Rng;

pub static BCHAR : char = ' ';

static TAIL_MIN : u32 = 4;
static TAIL_MAX : u32 = 18;

#[derive(Clone)]
struct Column {
    lead_index : usize,
    priority : usize,
    tail_length : usize,
    next_animation: u64,
}

impl Column {
    pub fn new(lead_index: usize, priority: usize, tail_length: usize, next_animation: u64) -> Self {
        Self { lead_index, priority, tail_length, next_animation }
    }
}

pub struct Cell {
    pub col: usize, // x
    pub row: usize, // y
    pub character: char,
}

pub struct Matrix {
    matrix : Vec<Vec<char>>,
    columns : Vec<Column>,
    pub dirty: Vec<Cell>,
}

impl Matrix {
    pub fn new(rows : usize, cols : usize) -> Self {
        let matrix = vec![vec![BCHAR; cols]; rows];
        let priorities: Vec<usize> = (5..50).collect();

        let mut columns = vec![];  
        for _ in 0..cols {
            let p = rand::thread_rng().gen_range(0..priorities.len() as u32) as usize;
            let tail = rand::thread_rng().gen_range(TAIL_MIN..TAIL_MAX as u32) as usize;
            columns.push(Column::new(0, priorities[p], tail, 100));
        }
        Self{ matrix, columns, dirty: vec![]}
    }

    pub fn num_cols(&self) -> usize {
        self.matrix[0].len()
    }

    pub fn num_rows(&self) -> usize {
        self.matrix.len()
    }

    pub fn col_priority(&self, col: usize) -> usize {
        self.columns[col].priority
    } 

    pub fn set_col_next_animation(&mut self, col: usize, next: u64) {
        self.columns[col].next_animation = next;
    }

    pub fn col_next_animation(&self, col: usize) -> u64{
        self.columns[col].next_animation
    }

    pub fn tail_length(&self, col: usize) -> usize {
        self.columns[col].tail_length
    }

    pub fn append_char_to_column(&mut self, col : usize, c : char) {
        let mut row = self.columns[col].lead_index;
        let mut last_row: Option<usize> = None;
        if row >= self.matrix.len() {
            // if we go over the view hieght then we don't reset yet to spread
            // out the streams from each other
            // but if we exceed the REAL bottom then we reset
            if row >= self.matrix.len() + TAIL_MAX as usize {
                self.columns[col].lead_index = 0;
                row = 0;
            } else {
                last_row = Some(self.num_rows()-1);
                self.columns[col].lead_index = row + 1;
            }
        } else {
            self.matrix[row][col] = c;
            if row > 0 {
                last_row = Some(row-1);
            }
            // lead char in stream (square) only needed in visible viewport
            let square_char = char::from_u32(0x2588 as u32).unwrap();
            self.dirty.push(Cell {col, row, character: square_char });
            self.columns[col].lead_index = row + 1;
        }

        if let Some(last_row) = last_row { // second letter in stream
            self.dirty.push( Cell{ col, row: last_row, character: c })
        }
       
        // tail char gets cleared (aka column fade)
        if let Some(tail) = row.checked_sub(self.tail_length(col)) {
            self.dirty.push(Cell {col, row: tail, character: BCHAR });
        }
    }

    pub fn overwrite_char(&mut self, row : usize, col : usize, c : char) {
        self.matrix[row][col] = c;
        self.dirty.push(Cell{ col, row, character: c })
    }

    pub fn lead_index(&self, col : usize) -> usize {
        self.columns[col].lead_index
    }
}
