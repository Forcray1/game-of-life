# Game of Life

A Rust project focused on learning how to model, simulate, and evolve a cellular automaton using the Game of Life rules.

## Overview

This project is intended as a learning exercise in Rust programming, data modeling, and simulation logic. The goal is to build a simple implementation of Conway's Game of Life and explore how a grid-based system evolves over time under a set of rules.

## Learning Goal

- Learn the Rust language through a practical project
- Practice working with data structures and state updates
- Implement simulation logic in a clean, understandable way
- Explore iteration, ownership, and performance considerations in Rust
- Build a small project that can be expanded over time

## Project Description

### Purpose

To simulate the Game of Life in Rust and provide a foundation for learning and experimentation.

### Scope

- Represent a grid of cells
- Update cells according to Game of Life rules
- Visualize or print the grid over time
- Allow experimentation with patterns and board size

## Project Structure

### Directory Layout

- `src/`
  - `main.rs`
- `Cargo.toml`
- `README.md`

### File Responsibilities

#### `src/main.rs`


#### `Cargo.toml`


## Core Concepts

### Game of Life Rules

Conway's Game of Life is a zero-player cellular automaton. Each cell in a grid is either alive or dead, and the board evolves in discrete time steps.

A cell follows these rules at each generation:

- A live cell with 2 neighbors remains alive.
- A live cell with 3 neighbors remains alive.
- A live cell with fewer than 2 neighbors dies from underpopulation.
- A live cell with more than 3 neighbors dies from overpopulation.
- A dead cell with exactly 3 neighbors becomes alive through reproduction.
- A dead cell with any other number of neighbors stays dead.

These rules are applied simultaneously to the entire board, meaning each generation is calculated from the previous state before any updates are applied.

### Cellular Automata


### Grid Representation


## Algorithm

### Simulation Loop


### Neighbor Counting


### State Transition


### Step-by-Step Update Process


## Data Structures

### Cell Representation


### Grid Representation


### Pattern Storage


## Implementation Plan

### Phase 1


### Phase 2


### Phase 3


## Usage

### Running the Project


### Example Workflow


## Testing

### Test Ideas


### Validation


## Notes

### Challenges to Explore


### Possible Extensions


## Future Improvements


## Conclusion

This project is a learning-focused Rust implementation of the Game of Life. It provides a simple but rich problem space for exploring simulation logic, state transitions, and Rust development practices.
