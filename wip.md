# WIP: Paint Loop Performance

## Goal
Only repaint cells that have changed, instead of reprinting the entire matrix every frame.

## Plan

### 1. Add dirty list to `Matrix`
Add `dirty: Vec<(usize, usize, char)>` to the `Matrix` struct.
Push to it inside `overwrite_char` and `append_char_to_column` at write time (include the char value so the paint thread never needs to re-read the matrix).

### 2. Update paint loop
- Acquire lock, drain dirty list with `std::mem::take(&mut matrix.dirty)`, release lock immediately
- Use `queue!` + crossterm `Print` to batch all dirty-cell writes
- Single `stdout.flush()` at end of frame

```rust
use crossterm::{queue, style::Print};

let dirty = {
    let mut matrix = matrix.lock().unwrap();
    std::mem::take(&mut matrix.dirty)
};
for (row, col, c) in dirty {
    queue!(stdout, cursor::MoveTo(col as u16, row as u16), Print(c)).unwrap();
}
stdout.flush().unwrap();
```

### 3. Remove the initial full-repaint setup
The `MoveTo(0, 0)` + full row iteration loop goes away entirely. The initial `Clear` on startup is still needed.

## Why

- **Dirty list over dirty matrix**: list stays small (O(cols) entries per frame at most), no full-matrix scan to find dirty bits
- **Char in dirty entry**: paint thread releases the mutex before doing any I/O, so the mutate thread isn't blocked during painting
- **`queue!` over `print!`**: `print!` acquires the stdout lock per call and doesn't flush; `execute!` flushes per call. `queue!` batches everything into a single flush
- **Eliminates `matrix.rows()` allocation**: current code allocates a `Vec<String>` (O(rows×cols)) every frame — gone with dirty list
- **Duplicate dirty entries**: possible but harmless (same cell written twice = redundant terminal write). No need for a HashSet
