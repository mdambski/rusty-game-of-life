# Rusty Game of Life
Conway's Game of Life in **Rust**. It’s a fun way for me to learn Rust while building a classic simulation.

## Features
- Interactive terminal-based simulation of Conway's Game of Life.
- Customizable grid size and simulation options.
- Random seeding of the initial state.
- Detection if game entered steady state or oscilation


## Prerequisites
If you haven't already, you need to install Rust. Follow [official installation guide](https://www.rust-lang.org/tools/install).

## Installation and Setup

1. Clone the repository:
```bash
git clone https://github.com/mdambski/rusty-game-of-life.git
cd rusty-game-of-life
```

2. Build the project:
```bash
cargo build --release
```

3. Run the simulation:
```bash
cargo run
```

## Usage
To customize the grid size, or simulation behaviour use provided command line options:

```bash
Options:
  -g, --grid-size <GRID_SIZE>  Grid size for the simulation [default: 30]
  -e, --exit-steady            Detect and stop at steady state or oscilation
```

Example:
```bash
cargo run -- --grid-size 50 --exit-steady
```

When running without steady state detection, press `Ctrl+C` to exit the simulation.

## Testing
Currently project has only unit tests covering game module. With time integration tests will be added, when the complexity grows.

## References
- Inspired by [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life).

