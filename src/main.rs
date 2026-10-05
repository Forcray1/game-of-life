use std::collections::HashMap;
use rand::RngExt;
use std::thread;
use std::time::Duration;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Cell {
	Dead,
	Alive,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Loc {
	x: u32,
	y: u32,
}

pub struct Grid {
	width: u32,
	height: u32,
	grid: HashMap<Loc,Cell>,
}

impl Grid{
	fn new(width: u32, height: u32, prob: f64) -> Self {
        let mut grid = HashMap::new();
		let mut rng = rand::rng();
        for y in 0..height {
            for x in 0..width {
				let cell = if rng.random_bool(prob) {
					Cell::Alive
				} else {
					Cell::Dead
				};
                grid.insert(Loc { x, y }, cell);
            }
        }
        Self { width, height, grid }
    }

	pub fn check_neighbour(&self, loc: Loc) -> u32 {
		let mut count: u32 = 0;
		for dx in [-1, 0, 1] {
			for dy in [-1, 0, 1]{
				if dx == 0 && dy == 0 {
					continue;
				}
				let nx = loc.x as i32 + dx;
				let ny = loc.y as i32 + dy;
				if nx < 0 || ny < 0 || nx >= self.width as i32 || ny >= self.height as i32 {
					continue;
				}
				let nloc = Loc { x: nx as u32, y: ny as u32 };
				if self.grid.get(&nloc) == Some(&Cell::Alive) {
					count += 1;
				}
			}
		}
		count
	}

	pub fn turn(&mut self) {
		let mut next = HashMap::new();
		for (loc, cell) in &self.grid {
			let n = self.check_neighbour(*loc);
			if n > 3 || n < 2 {
				next.insert(*loc, Cell::Dead);
			} else {
				next.insert(*loc, Cell::Alive);
			}
		}
		self.grid = next;
	}

	pub fn print_grid(&self) {
		for y in 0..self.height {
			let mut line = String::new();
			for x in 0..self.width {
				if self.grid.get(&Loc { x, y }) == Some(&Cell::Alive) {
					line.push_str("# ");
				} else {
					line.push_str(". ");
				}
			}
			println!("{}", line);
		}
	}
}

fn main() {
    let width = 100;
	let height = 100;
	let prob = 0.1;

	let mut grid = Grid::new(width, height, prob);

    for _ in 0..100 {
        print!("\x1B[2J\x1B[H");
        grid.print_grid();
        grid.turn();
        thread::sleep(Duration::from_millis(3000));
    }
	
}
