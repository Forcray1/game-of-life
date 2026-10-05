#[derive(Clone, Copy, PartialEq, Eq)]
enum Cell {
	Dead,
	Alive,
}

enum Grid {
	width: usize,
	height: usize,
	cells: Vec<Cell>
}

impl Grid{
	fn new(width: usize, height: usize) -> Self {
        Self { width, height, cells: vec![Cell::Dead; width * height] }
    }

    fn idx(&self, row: usize, col: usize) -> usize {
        row * self.width + col
    }
}

fn main() {
    println!("Hello, world!");
}
