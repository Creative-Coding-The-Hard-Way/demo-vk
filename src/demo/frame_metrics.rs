use {
    super::rolling_average::RollingAverage,
    std::{collections::BTreeMap, time::Instant},
};

#[derive(Debug)]
pub struct FrameMetrics {
    target_fps: usize,
    last_frame: Instant,
    ms_per_frame: RollingAverage,
    ms_per_update: RollingAverage,
    ms_per_draw: RollingAverage,
    metrics: BTreeMap<String, RollingAverage>,
}

impl FrameMetrics {
    /// Creates a new FrameMetrics instance.
    pub(super) fn new(target_fps: usize) -> Self {
        Self {
            target_fps,
            last_frame: Instant::now(),
            ms_per_frame: RollingAverage::new(
                target_fps,
                1.0 / target_fps as f32,
            ),
            ms_per_update: RollingAverage::new(
                target_fps,
                1.0 / target_fps as f32,
            ),
            ms_per_draw: RollingAverage::new(
                target_fps,
                1.0 / target_fps as f32,
            ),
            metrics: BTreeMap::new(),
        }
    }

    /// Called when the application is unpaused.
    ///
    /// Resets the last frame time so there isn't a single massively slow frame.
    pub(super) fn unpause(&mut self) {
        self.last_frame = Instant::now();
    }

    /// Calculates the time since the last frame and update internal metrics.
    ///
    /// Returns the time since the last frame in seconds.
    pub(super) fn frame_tick(&mut self) -> f32 {
        let now = Instant::now();
        let frame_seconds = now.duration_since(self.last_frame).as_secs_f32();
        self.last_frame = now;
        self.ms_per_frame.push(frame_seconds * 1000.0);

        frame_seconds
    }

    /// Records the frame's update time in milliseconds.
    pub(super) fn update_tick(&mut self, before_update: Instant) {
        let ms =
            Instant::now().duration_since(before_update).as_secs_f32() * 1000.0;
        self.ms_per_update.push(ms);
    }

    /// Records the frame's draw time in milliseconds.
    pub(super) fn draw_tick(&mut self, before_draw: Instant) {
        let ms =
            Instant::now().duration_since(before_draw).as_secs_f32() * 1000.0;
        self.ms_per_draw.push(ms);
    }

    /// Records the milliseconds from start_time until now() at the time this
    /// function is called.
    ///
    /// Returns the milliseconds since the start time.
    pub fn ms_since(
        &mut self,
        name: impl Into<String>,
        start_time: Instant,
    ) -> f32 {
        let ms =
            Instant::now().duration_since(start_time).as_secs_f32() * 1000.0;
        self.record_metric(name, ms);
        ms
    }

    /// Saves a duration to the frame metrics.
    pub fn record_metric(&mut self, name: impl Into<String>, value: f32) {
        let metric = self
            .metrics
            .entry(name.into())
            .or_insert_with(|| RollingAverage::new(self.target_fps, value));
        metric.push(value);
    }

    /// Resets all tracked metrics.
    pub fn reset_metrics(&mut self) {
        self.metrics.clear();
    }
}

impl std::fmt::Display for FrameMetrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let precision = f.precision().unwrap_or(2);
        f.write_fmt(format_args!(
            indoc::indoc! {"
                Frame Metrics
                fps : {:.0}
                mspf: {:.*}
                mspu: {:.*}
                mspd: {:.*}
            "},
            1000.0 / self.ms_per_frame.average(),
            precision,
            self.ms_per_frame,
            precision,
            self.ms_per_update,
            precision,
            self.ms_per_draw,
        ))?;

        for (name, metric) in self.metrics.iter() {
            f.write_fmt(format_args!("{}: {:.*}\n", name, precision, metric))?;
        }

        Ok(())
    }
}
