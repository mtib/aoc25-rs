# aoc25

Advent of Code 2025 solutions in Rust.

## Usage

```bash
# Run the latest day (all parts, example + actual input)
cargo run

# Run specific day(s)
cargo run 1        # Day 1, both parts, example + actual
cargo run 1 2 3    # Days 1, 2, and 3

# Run specific part
cargo run 3.1      # Day 3, part 1 only
cargo run 3.2      # Day 3, part 2 only

# Run with specific input type
cargo run 3e       # Day 3, example input only
cargo run 3a       # Day 3, actual input only

# Combine options
cargo run 3.1e     # Day 3, part 1, example input
cargo run 3.2a     # Day 3, part 2, actual input

# Run all days
cargo run e        # All days, example input only
cargo run a        # All days, actual input only
cargo run e a      # All days, both example and actual input
```

## Setup

Create a `.env` file with your Advent of Code session cookie:

```
AOC_SESSION=your_session_cookie_here
```
