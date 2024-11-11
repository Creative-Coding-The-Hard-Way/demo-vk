use std::collections::VecDeque;

#[derive(Debug)]
pub struct RollingAverage {
    values: VecDeque<f32>,
    average: f32,
    min: f32,
    max: f32,
}

impl RollingAverage {
    pub fn new(count: usize, initial_value: f32) -> Self {
        let mut values = VecDeque::with_capacity(count);
        for _ in 0..count {
            values.push_back(initial_value);
        }
        Self {
            values,
            average: initial_value,
            min: initial_value,
            max: initial_value,
        }
    }

    pub fn average(&self) -> f32 {
        self.average
    }

    pub fn min(&self) -> f32 {
        self.min
    }

    pub fn max(&self) -> f32 {
        self.max
    }

    pub fn push(&mut self, value: f32) {
        self.values.pop_front();
        self.values.push_back(value);

        let mut max = self.values[0];
        let mut min = self.values[0];
        let mut sum = 0.0;
        for &value in &self.values {
            sum += value;
            if value < min {
                min = value;
            }
            if value > max {
                max = value;
            }
        }
        self.average = sum / self.values.len() as f32;
        self.min = min;
        self.max = max;
    }
}

impl std::fmt::Display for RollingAverage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let precision = f.precision().unwrap_or(2);
        f.write_fmt(format_args!(
            "{:.*} | [{:.*}, {:.*}]",
            precision,
            self.average(),
            precision,
            self.min(),
            precision,
            self.max()
        ))
    }
}
