use std::{error::Error, fmt::Display};

#[derive(Debug, Clone)]
pub struct PuzzleNotImplementedError;

impl Display for PuzzleNotImplementedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Puzzle not implemented")
    }
}

impl Error for PuzzleNotImplementedError {}

#[derive(Debug, Clone)]
pub struct ExampleInputNotAvailableError;

impl Display for ExampleInputNotAvailableError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Example input not available")
    }
}

impl Error for ExampleInputNotAvailableError {}
