Conway's Game of Life in **Rust**. It’s a fun way for me to learn Rust while building a classic simulation.

### What's in it?

So far, it's really simple. It generates a random seed and runs the simulation.

### Running the Simulation

```bash
git clone https://github.com/mdambski/rusty-game-of-life.git
cd rusty-game-of-life
cargo run
```

Supported parameters:

```
Options:
  -g, --grid-size <GRID_SIZE>  Grid size for the simulation [default: 30]
  -e, --exit-steady            Detect and stop at steady state or oscilation
```


work in progress