use std::ops::RangeInclusive;
use num::{Float, Num, traits::real::Real};

// Remap a number within range `old` to a number in range `new`
// Example - map_range(5, 0..=10, -10..=10) == 0
// Restricting this to floats to avoid rounding errors. I mean, unsoundly big rounding errors.
pub fn map_range<TFloat: Float + Copy>(nmb: TFloat, old: RangeInclusive<TFloat>, new: RangeInclusive<TFloat>) -> TFloat {
    let ratio = (*new.end() - *new.start()) / (*old.end() - *old.start());
    let new = (nmb - *old.start()) * ratio + *new.start();
    new
}

pub fn approx_eq<TNum: Num + Real>(lhs: TNum, rhs: TNum, margin: TNum) -> bool {
    lhs == rhs || (lhs - rhs).abs() <= margin
}

macro_rules! assert_approx_eq {
    ($lhs:expr, $rhs:expr, $margin:expr) => {
        assert!(crate::utils::num::approx_eq($lhs as f64, $rhs as f64, $margin as f64))
    };
}

pub(crate) use assert_approx_eq;

#[cfg(test)]
mod tests {
    use itertools::izip;

    use crate::utils::tests::Case;

    use super::*;

    /// 0..=10 -> -1..=0
    #[test]
    fn map_range_gradual() {
        let mapped = (0..=10).map(|x| {
            map_range(x as f64, 0.0..=10.0, -1.0..=0.0)
        });
        let expected = [-1.0, -0.9, -0.8, -0.7, -0.6, -0.5, -0.4, -0.3, -0.2, -0.1, -0.0];
        for (actual, expected) in izip!(mapped, expected) {
            eprintln!("Actual: {actual}, expected: {expected}");
            assert_approx_eq!(actual, expected, 0.01)
        }
    }

    #[test]
    fn map_range_handpicked() {
        const CASES: [Case<(f64, RangeInclusive<f64>, RangeInclusive<f64>), f64>; 3] = [
            Case { payload: (5.0, 0.0..=10.0, -10.0..=10.0), expected: 0.0 }, 
            Case { payload: (0.0, 0.0..=10.0, 50.0..=100.0), expected: 50.0 }, 
            Case { payload: (27.0, 14.0..=28.0, -100.0..=100.0), expected: 86.0 }, 
        ];

        for Case { payload, expected } in CASES {
            let res = map_range(payload.0, payload.1, payload.2);
            eprintln!("Actual: {res}, expected: {expected}");
            assert_approx_eq!(res, expected, 1.0);
        }
    }
}