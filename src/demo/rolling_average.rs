use std::collections::VecDeque;

#[derive(Debug)]
pub struct RollingAverage {
    values: VecDeque<f32>,
    count: f32,
    sum: f32,
    sum_of_squares: f32,
}

impl RollingAverage {
    pub fn new(count: usize, initial_value: f32) -> Self {
        let mut values = VecDeque::with_capacity(count);
        for _ in 0..count {
            values.push_back(initial_value);
        }
        Self {
            values,
            count: count as f32,
            sum: initial_value * count as f32,
            sum_of_squares: (initial_value * initial_value) * count as f32,
        }
    }

    pub fn average(&self) -> f32 {
        self.sum / self.count
    }

    pub fn std_dev(&self) -> f32 {
        (1.0 / (self.count - 1.0))
            * (self.sum_of_squares - (self.sum * self.sum) / self.count)
    }

    pub fn push(&mut self, value: f32) {
        let old_value = self.values.pop_front().unwrap();
        self.values.push_back(value);

        // incrementally update sum and sum_of_squares
        self.sum += value - old_value;
        self.sum_of_squares += (value * value) - (old_value * old_value);
    }
}

impl std::fmt::Display for RollingAverage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let precision = f.precision().unwrap_or(2);
        f.write_fmt(format_args!(
            "{:.*}±{:.*}",
            precision,
            self.average(),
            precision,
            self.std_dev(),
        ))
    }
}
