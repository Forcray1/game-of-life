# Game of Life

A terminal-based Conway's Game of Life simulation written in Rust, featuring a randomly seeded grid and real-time rendering in the terminal.

## Overview

This project simulates Conway's Game of Life directly in the terminal. The board is randomly initialized with a configurable alive-cell probability and evolves over 100 generations, re-rendering each step using ANSI escape codes.

## Learning Goals

- Learn the Rust language through a practical project
- Practice working with `HashMap`-based data structures and state updates
- Implement simulation logic cleanly with structs and methods
- Explore ownership, iteration, and performance considerations in Rust

## Project Description

### Purpose

To simulate the Game of Life in Rust and provide a foundation for learning and experimentation.

### Scope

- Represent a grid of cells backed by a `HashMap<Loc, Cell>`
- Update cells according to Game of Life rules each generation
- Render the grid in the terminal with `#` (alive) and `.` (dead)
- Allow configuration of board size and initial alive probability

## Project Structure

### Directory Layout

```
src/
  main.rs
Cargo.toml
README.md
```

### File Responsibilities

#### `src/main.rs`

Contains all logic: the `Cell` enum, the `Loc` struct, the `Grid` struct with its methods (`new`, `check_neighbour`, `turn`, `print_grid`), and the `main` entry point.

#### `Cargo.toml`

Declares the crate (`game-of-life`, edition 2024) and the single dependency: `rand = "0.10.3"` for random cell seeding.

## Core Concepts

### Game of Life Rules

Conway's Game of Life is a zero-player cellular automaton. Each cell in a grid is either alive or dead, and the board evolves in discrete time steps.

A cell follows these rules at each generation:

- A live cell with 2 or 3 neighbors remains alive.
- A live cell with fewer than 2 neighbors dies from underpopulation.
- A live cell with more than 3 neighbors dies from overpopulation.
- A dead cell with exactly 3 neighbors becomes alive through reproduction.
- Any other dead cell stays dead.

These rules are applied simultaneously to the entire board using the **previous** state.

> **Note:** the current `turn()` implementation sets any cell with exactly 2 live neighbors to `Dead` regardless of its current state (a known bug — a living cell with 2 neighbors should survive).

### Cellular Automata

A cellular automaton is a discrete model where a grid of cells evolves through generations based on fixed local rules applied uniformly across the grid.

### Grid Representation

The grid is stored as a `HashMap<Loc, Cell>` where `Loc` is a `(x, y)` coordinate struct. This avoids a 2D vector and makes neighbor lookups straightforward.

## Algorithm

### Simulation Loop

```
for 100 generations:
    clear terminal (ANSI \x1B[2J\x1B[H)
    print_grid()
    turn()
    sleep 3 seconds
```

### Neighbor Counting

`check_neighbour(loc)` iterates over all 8 surrounding `(dx, dy)` offsets, skips `(0, 0)`, clamps to grid bounds, and counts `Alive` neighbors.

### State Transition

`turn()` builds a new `HashMap` from the current state:
- Cells with fewer than 2 or more than 3 alive neighbors → `Dead`
- All other cells → `Alive`

The new map replaces `self.grid` atomically.

### Step-by-Step Update Process

1. Iterate over every `(loc, cell)` pair in the current grid.
2. Count alive neighbors via `check_neighbour`.
3. Apply rules to determine next state.
4. Swap the grid with the newly computed map.

## Data Structures

### Cell Representation

```rust
pub enum Cell { Dead, Alive }
```

Derives `Clone`, `Copy`, `PartialEq`, `Eq`, `Debug`.

### Grid Representation

```rust
pub struct Grid {
    width: u32,
    height: u32,
    grid: HashMap<Loc, Cell>,
}
```

### Loc (Coordinate)

```rust
pub struct Loc { x: u32, y: u32 }
```

Derives `Clone`, `Copy`, `PartialEq`, `Eq`, `Hash`, `Debug` — required to be used as a `HashMap` key.

## Usage

### Running the Project

```bash
cargo run
```

This starts a 100×100 grid initialized with ~10% alive cells and runs for 100 generations, printing each one to the terminal with a 3-second delay between steps.

### Configuration

All parameters are hardcoded in `main()`:

| Parameter | Value | Description |
|-----------|-------|-------------|
| `width`   | 100   | Grid width  |
| `height`  | 100   | Grid height |
| `prob`    | 0.1   | Probability of a cell starting alive |

## Testing

### Test Ideas

- Verify `check_neighbour` returns the correct count for known patterns (e.g., blinker, block).
- Verify `turn()` correctly evolves a stable block pattern (should not change).
- Verify underpopulation and overpopulation kill cells as expected.

### Validation

The `turn()` bug (2-neighbor cells always dying) can be caught by testing a 2×2 alive block — it should remain stable but currently dies after one generation.

## Notes

### Known Issues

- `turn()` marks cells with exactly 2 neighbors as `Dead` unconditionally, which breaks the "live cell with 2 neighbors survives" rule.

### Possible Extensions

- Fix the `turn()` survival rule.
- Make width, height, and probability configurable via CLI arguments.
- Add pattern presets (glider, blinker, etc.).
- Replace the sleep-based loop with a key-press-driven step mode.
- Wrap edges (toroidal grid).

## Future Improvements

- CLI argument parsing with `clap`.
- Configurable frame delay.
- Persistent pattern files (RLE format).
- Multithreaded generation computation for large grids.

## Conclusion

This project is a learning-focused Rust implementation of Conway's Game of Life. It covers core Rust concepts — enums, structs, `HashMap`, iterators, and ownership — in the context of a simple but rich simulation problem.
