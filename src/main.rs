// External crate imports
use clap::Parser;

// Modules
mod game;


/// Command-line arguments parser
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Grid size for the simulation
    #[arg(short, long, default_value_t = 30, value_parser = validate_grid_size)]
    grid_size: usize,

    /// Detect and stop at steady state or oscilation
    #[arg(short, long, default_value_t = false)]
    exit_steady: bool
}

fn validate_grid_size(value: &str) -> Result<usize, String> {
    let grid_size: usize = value
        .parse()
        .map_err(|_| format!("`{}` isn't a valid number", value))?;

    if (1..=100).contains(&grid_size) {
        Ok(grid_size)
    } else {
        Err(format!("Grid size must be between 1 and 100, but got {}", grid_size))
    }
}

/// Main simulation loop.
fn main() {    
    let args = Args::parse(); 
    game::run_simulation(args.grid_size, args.exit_steady);
}
