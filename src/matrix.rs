
use rand;
use rand::Rng;

pub static BCHAR : char = ' ';

static PRIORITY_MIN : u32 = 20;
static PRIORITY_MAX : u32 = 1;
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

pub struct Matrix {
    matrix : Vec<Vec<char>>,
    columns : Vec<Column>,
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
        Self{ matrix, columns }
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
        if row >= self.matrix.len() {
            if row >= self.matrix.len() + TAIL_MAX as usize {
                self.columns[col].lead_index = 0;
                row = 0;
            }
        } else {
            self.matrix[row][col] = c;
        }
        self.columns[col].lead_index = row + 1;
    }

    pub fn overwrite_char(&mut self, row : usize, col : usize, c : char) {
        self.matrix[row][col] = c;
    }

    pub fn clear_col(&mut self, col : usize) {
        for row in self.matrix.iter_mut() {
            row[col] = BCHAR;
        }
    }

    pub fn remove_row(&mut self, num : u8) {
        self.matrix.remove(num as usize);
        self.matrix.push(vec![BCHAR; self.num_cols()]);
        for j in self.columns.iter_mut() {
            if j.lead_index > 0 {
                j.lead_index -= 1;
            }
        }
    }

    pub fn lead_index(&self, col : usize) -> usize {
        self.columns[col].lead_index
    }

    pub fn rows(&self) -> Vec<String> {
        let mut res: Vec<String> = vec![];
        for row in self.matrix.iter() {
            res.push(row.into_iter().collect());
        }
        res 
    }
}
