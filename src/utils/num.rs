use num;
use std::ops::RangeInclusive;

#[cfg(test)]
mod tests;

// Remap a number within range `old` to a number in range `new`
// Example - map_range(5, 0..=10, -10..=10) == 0
// Number is cast into f64 (long) for operation.
pub fn map_range(
    nmb: impl Into<f64> + Copy,
    old: RangeInclusive<impl Into<f64> + Copy>,
    new: RangeInclusive<impl Into<f64> + Copy>,
) -> f64 {
    let old: (f64, f64) = ((*old.start()).into(), (*old.end()).into());
    let new: (f64, f64) = ((*new.start()).into(), (*new.end()).into());
    let ratio = (new.1 - new.0) / (old.1 - old.0);
    let new = (Into::<f64>::into(nmb) - old.0) * ratio + new.0;
    new
}

pub fn approx_eq<Float: num::Float + Copy>(lhs: Float, rhs: Float, margin: Float) -> bool {
    lhs == rhs || (lhs - rhs).abs() <= margin
}