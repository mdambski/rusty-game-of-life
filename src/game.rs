use std::collections::VecDeque;
use std::io::{Write};
use std::{thread, time};
use rand::Rng;

const MAX_HISTORY: usize = 10;
const MAX_ITERATIONS: i32 = i32::MAX;
const SLEEP_DURATION: time::Duration = time::Duration::from_millis(50);
const NEIGHBOR_OFFSETS: [(i32, i32); 8] = [
    (-1, -1), (-1, 0), (-1, 1),
    (0, -1),          (0, 1),
    (1, -1), (1, 0), (1, 1),
];

type Grid = Vec<Vec<bool>>;

/// Initializes the grid with all cells set to `false`.
fn initialize_grid(grid_size: usize) -> Grid {
    vec![vec![false; grid_size]; grid_size]
}

/// Runs the Conway's Game of Life simulation.
pub fn run_game_of_life_simulation(grid_size: usize, exit_steady: bool) {
    clear_screen();

    let mut grid = initialize_grid(grid_size);
    let mut new_grid = initialize_grid(grid_size);
    let mut history: VecDeque<Grid> = VecDeque::with_capacity(MAX_HISTORY);

    seed_grid_with_random_cells(&mut grid);

    for iteration in 0..MAX_ITERATIONS {
        display_grid(&grid, iteration);
        
        if exit_steady && detect_steady_state(&grid, &mut history) {
            println!("Repeating or steady state detected. Terminating at iteration {}.", iteration);
            break;
        }

        compute_next_generation(&grid, &mut new_grid);
        std::mem::swap(&mut grid, &mut new_grid);
    
        thread::sleep(SLEEP_DURATION);
    }
}

/// Detects if the current grid state matches any state in the history, indicating a steady state or oscillation.
fn detect_steady_state(grid: &Grid, history: &mut VecDeque<Grid>) -> bool {
    if history.contains(grid) {
        return true;
    }
    if history.len() == MAX_HISTORY {
        history.pop_front();
    }
    history.push_back(grid.clone());
    false
}

/// Computes the next generation of the grid based on the current state.
fn compute_next_generation(current: &Grid, next: &mut Grid) {
    let size = current.len();

    for row in 0..size {
        for col in 0..size {    
            let live_neighbors = count_live_neighbors(current, row, col);
            next[row][col] = match (current[row][col], live_neighbors) {
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
    let size = grid.len() as i32;
    NEIGHBOR_OFFSETS.iter().filter(|&&(dx, dy)| {
        let nx = row as i32 + dx;
        let ny = col as i32 + dy;
        nx >= 0 && ny >= 0 && nx < size && ny < size && grid[nx as usize][ny as usize]
    }).count() as i32
}

/// Clears the terminal screen.
fn clear_screen() {
    print!("\x1B[2J\x1B[H");
    std::io::stdout().flush().expect("Failed to flush stdout");
}

/// Displays the grid in the terminal.
fn display_grid(grid: &Grid, iteration: i32) {
    print!("\x1B[H");

    let mut output = String::with_capacity(grid.len() * (grid.len() + 1) + 20);
    for row in grid {
        for &cell in row {
            output.push_str(if cell { "# " } else { ". " });
        }
        output.push('\n');
    }
    output.push_str(&format!("Iteration: {}\n", iteration));

    print!("{}", output);
    std::io::stdout().flush().expect("Failed to flush stdout");
}

/// Seeds the grid with random live cells.
fn seed_grid_with_random_cells(grid: &mut Grid) {
    let size = grid.len();
    let mut rng = rand::thread_rng();
    let live_cells = rng.gen_range(size..=(size * size) / 5);

    for _ in 0..live_cells {
        let x: usize = rng.gen_range(0..size);
        let y: usize = rng.gen_range(0..size);
        grid[x][y] = true;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_grid() {
        let size = 5;
        let grid = initialize_grid(size);

        assert_eq!(grid.len(), size);
        for row in &grid {
            assert_eq!(row.len(), size);
            assert!(row.iter().all(|&cell| !cell));
        }
    }

    #[test]
    fn test_detect_steady_state() {
        let mut history = VecDeque::with_capacity(MAX_HISTORY);
        let grid = vec![vec![true, false], vec![false, true]];

        // Initially, the grid is not in history
        assert!(!detect_steady_state(&grid, &mut history));

        // After being added, the grid is in history
        assert!(detect_steady_state(&grid, &mut history));
    }

    #[test]
    fn test_compute_next_generation() {
        let current = vec![
            vec![false, true, false],
            vec![false, true, false],
            vec![false, true, false],
        ];
        let mut next = initialize_grid(3);

        compute_next_generation(&current, &mut next);

        let expected = vec![
            vec![false, false, false],
            vec![true, true, true],
            vec![false, false, false],
        ];
        assert_eq!(next, expected);
    }

    #[test]
    fn test_count_live_neighbors() {
        let grid = vec![
            vec![true, false, true],
            vec![false, true, false],
            vec![true, false, true],
        ];

        assert_eq!(count_live_neighbors(&grid, 1, 1), 4);
        assert_eq!(count_live_neighbors(&grid, 0, 0), 1);
        assert_eq!(count_live_neighbors(&grid, 0, 2), 1);
    }

    #[test]
    fn test_edge_case_empty_grid() {
        let grid = initialize_grid(0);

        // Detecting steady state should handle empty grid gracefully
        let mut history = VecDeque::new();
        assert!(!detect_steady_state(&grid, &mut history));

        // Compute next generation on empty grid
        let mut next = initialize_grid(0);
        compute_next_generation(&grid, &mut next);
        assert_eq!(next.len(), 0);
    }

    #[test]
    fn test_edge_case_single_cell() {
        let grid = vec![vec![true]];
        let mut next = initialize_grid(1);

        compute_next_generation(&grid, &mut next);

        // Single live cell dies in the next generation
        assert_eq!(next, vec![vec![false]]);
    }
}