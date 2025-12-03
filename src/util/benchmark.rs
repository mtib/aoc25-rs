pub trait Benchmarker {
    fn start_benchmark(&mut self);
    fn end_benchmark(&mut self);
    /// Returns the benchmark time in milliseconds
    fn elapsed_ms(&self) -> Option<f64>;
    fn n(&self) -> usize;
}

pub struct SimpleBenchmarker {
    start_time: Option<std::time::Instant>,
    durations: Vec<std::time::Duration>,
}

impl SimpleBenchmarker {
    pub fn new() -> Self {
        Self {
            start_time: None,
            durations: vec![],
        }
    }
}

impl Benchmarker for SimpleBenchmarker {
    fn start_benchmark(&mut self) {
        self.start_time = Some(std::time::Instant::now());
    }

    fn end_benchmark(&mut self) {
        if let Some(start) = self.start_time {
            let duration = start.elapsed();
            self.durations.push(duration);
            self.start_time = None;
        }
    }

    fn elapsed_ms(&self) -> Option<f64> {
        if self.durations.is_empty() {
            None
        } else {
            let total_duration: std::time::Duration = self.durations.iter().copied().sum();
            let avg_duration = total_duration / (self.durations.len() as u32);
            Some(avg_duration.as_secs_f64() * 1000.0)
        }
    }

    fn n(&self) -> usize {
        self.durations.len()
    }
}
