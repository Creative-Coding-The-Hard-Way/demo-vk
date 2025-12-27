use std::ops::{Add, Div, Mul, Range, Sub};

pub mod app;
pub mod demo;
pub mod graphics;

pub fn map<T>(x: T, input_range: Range<T>, output_range: Range<T>) -> T
where
    T: Copy
        + Sub<Output = T>
        + Add<Output = T>
        + Div<Output = T>
        + Mul<Output = T>,
{
    let (u, v) = (input_range.start, input_range.end);
    let (s, t) = (output_range.start, output_range.end);
    let m = (s - t) / (u - v);
    let b = (t * u - v * s) / (u - v);
    x * m + b
}

/// Creates a "Location" type that displays the current location in code when
/// printed.
#[macro_export]
macro_rules! here {
    () => {
        $crate::app::Location {
            file: file!(),
            line: line!(),
            col: column!(),
        }
    };
}

/// Tries to resolve a result into a successful output (much like the `?`
/// operator). Unlike the `?` operator, this macro adds context about the
/// current line to the error message if something goes wrongw.
#[macro_export]
macro_rules! unwrap_here {
    ($description:expr, $operation:expr) => {{
        use {anyhow::Context, $crate::here};
        { $operation }
            .with_context(|| format!("{} - {}", here!(), $description))?
    }};
}
