// Standard library imports
use std::collections::VecDeque;
use std::io::{Write};
use std::{thread, time};

// External crate imports
use rand::Rng;

const MAX_HISTORY: usize = 10;
const MAX_ITERATIONS: i32 = i32::MAX;
const SLEEP_PER_ITERATION_MS: u64 = 50;

const NEIGHBOR_OFFSETS: [(i32, i32); 8] = [
    (-1, -1), (-1, 0), (-1, 1),
    (0, -1),            (0, 1),
    (1, -1), (1, 0), (1, 1),
];

type Grid = Vec<Vec<bool>>;

/// Initializes the grid with all cells set to `false`.
fn initialize_grid(grid_size: usize) -> Grid {
    vec![vec![false; grid_size]; grid_size]
}

pub fn run_simulation(grid_size: usize, exit_steady: bool) {
    
    clear_screen();

    let mut grid = initialize_grid(grid_size);
    let mut new_grid = initialize_grid(grid_size);
    let mut history: VecDeque<Grid> = VecDeque::with_capacity(MAX_HISTORY);

    // Make random seed
    seed(&mut grid);

    // Run simulation
    for iteration in 0..MAX_ITERATIONS {
        display_grid(&grid, iteration);
        
        if exit_steady {
            // Check for steady state or oscillation
            if history.contains(&grid) {
                println!(
                    "Repeating or steady state detected. Terminating at iteration {}.",
                    iteration
                );
                break;
            }
            
            // Add current state to history
            if history.len() == MAX_HISTORY {
                history.pop_front(); // Remove oldest state to maintain size
            }
            history.push_back(grid.clone());
        }

        // Compute new generation state based on the current
        compute_next_generation(&grid, &mut new_grid);
        
        // Swap grids instead of copying or resetting
        std::mem::swap(&mut grid, &mut new_grid);
    
        thread::sleep(time::Duration::from_millis(SLEEP_PER_ITERATION_MS));
    }
}

/// Computes the next generation of the grid based on the current state.
fn compute_next_generation(grid: &Grid, new_grid: &mut Grid) {

    let rows = grid.len();
    let cols = grid[0].len();

    for row in 0..rows {
        for col in 0..cols {    
            let live_neighbors = count_live_neighbors(grid, row, col);
            new_grid[row][col] = match (grid[row][col], live_neighbors) {
                // Live cell survives with 2 or 3 neighbors.
                (true, 2) | (true, 3) => true,
                // Live cell dies with other number of neighbors.
                (true, _) => false,
                // Dead cell becomes alive with exactly 3 neighbors.
                (false, 3) => true,
                // Dead cell stays dead otherwise.
                (false, _) => false,
            };
        }
    }
}

/// Counts how many of the neighboring cells are alive.
fn count_live_neighbors(grid: &Grid, row: usize, col: usize) -> i32 {
    let mut count = 0;
    let max_coord = grid.len() as i32;
    for (dx, dy) in NEIGHBOR_OFFSETS.iter() {
        let nx = row as i32 + dx;
        let ny = col as i32 + dy;
        if nx >= 0 && ny >= 0 && ny < max_coord && nx < max_coord && grid[nx as usize][ny as usize] {
            count += 1;
        }
    };
    count
}


/// Clears the terminal screen.
fn clear_screen() {
    print!("\x1B[2J\x1B[H");
    std::io::stdout().flush().expect("Failed to flush stdout");
}

/// Displays the grid in the terminal.
fn display_grid(grid: &Grid, iteration: i32) {

    // use ANSI escape sequence
    print!("\x1B[H");

    let mut output = String::with_capacity(grid.len() * (grid.len() + 1) + 20);
    for row in grid {
        for &cell in row {
            output.push_str(if cell { "# " } else { "- " });
        }
        output.push('\n');
    }

    // Append the iteration info at the end
    output.push_str(&format!("Iteration: {}\n", iteration));

    // Print the entire buffered output at once
    print!("{}", output);

    // Flush stdout to ensure the buffer is immediately written to the terminal
    std::io::stdout().flush().expect("Failed to flush stdout");
}


fn seed(grid: &mut Grid) {
    let grid_size = grid.len();
    let mut rng = rand::thread_rng();
    let seed_number = rng.gen_range(grid_size..grid_size.pow(2)/5);

    for _ in 0..seed_number {
        let x: usize = rng.gen_range(0..grid_size);
        let y: usize = rng.gen_range(0..grid_size);
        grid[x][y] = true;
    }
}
