use std::cell::RefCell;

pub trait Benchmarker {
    fn start_benchmark(&self);
    fn end_benchmark(&self);
    /// Returns the benchmark time in milliseconds
    fn elapsed_ms(&self) -> Option<f64>;
}

pub struct SimpleBenchmarker {
    start_time: RefCell<Option<std::time::Instant>>,
    end_time: RefCell<Option<std::time::Instant>>,
}

impl SimpleBenchmarker {
    pub fn new() -> Self {
        Self {
            start_time: RefCell::new(None),
            end_time: RefCell::new(None),
        }
    }
}

impl Benchmarker for SimpleBenchmarker {
    fn start_benchmark(&self) {
        self.start_time
            .borrow_mut()
            .replace(std::time::Instant::now());
    }

    fn end_benchmark(&self) {
        self.end_time
            .borrow_mut()
            .replace(std::time::Instant::now());
    }

    fn elapsed_ms(&self) -> Option<f64> {
        let start = self.start_time.borrow();
        let end = self.end_time.borrow();
        if let (Some(s), Some(e)) = (*start, *end) {
            let duration = e.duration_since(s);
            Some(duration.as_secs_f64() * 1000.0)
        } else {
            None
        }
    }
}
