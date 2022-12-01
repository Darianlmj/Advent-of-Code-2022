use advent_day::{AdventDay, AdventDayProcessor};
use std::env::args;
use tracing::{error, info};

mod advent_day;
mod solutions;

pub fn main() {
    // Initialize the logger
    tracing_subscriber::fmt::init();

    // Process command line arguments
    let day = match args().nth(1) {
        Some(day) => day,
        None => {
            info!("Usage: cargo run -- <day_number>");
            return;
        }
    };

    // Instantiate a new AdventDay object
    let advent = match day.parse::<u8>() {
        Ok(num) => AdventDay::new(num),
        Err(_) => {
            error!("Day number must be a number");
            return;
        }
    };

    // Execute the day's puzzle
    advent.execute()
}
