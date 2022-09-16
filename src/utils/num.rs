use num;
use std::ops::RangeInclusive;

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

#[cfg(test)]
macro_rules! assert_approx_eq {
    ($lhs:expr, $rhs:expr, $margin:expr) => {
        assert!(crate::utils::num::approx_eq(
            $lhs as f64,
            $rhs as f64,
            $margin as f64
        ))
    };
}

#[cfg(test)]
pub(crate) use assert_approx_eq;

#[cfg(test)]
mod tests {
    use itertools::izip;

    use crate::utils::tests::Case;

    use super::*;

    /// 0..=10 -> -1..=0
    #[test]
    fn map_range_gradual() {
        let mapped = (0..=10).map(|x| map_range(x as f64, 0.0..=10.0, -1.0..=0.0));
        let expected = [
            -1.0, -0.9, -0.8, -0.7, -0.6, -0.5, -0.4, -0.3, -0.2, -0.1, -0.0,
        ];
        for (actual, expected) in izip!(mapped, expected) {
            eprintln!("Actual: {actual}, expected: {expected}");
            assert_approx_eq!(actual, expected, 0.01)
        }
    }

    #[test]
    fn map_range_handpicked() {
        const CASES: [Case<(f64, RangeInclusive<f64>, RangeInclusive<f64>), f64>; 3] = [
            Case {
                payload: (5.0, 0.0..=10.0, -10.0..=10.0),
                expected: 0.0,
            },
            Case {
                payload: (0.0, 0.0..=10.0, 50.0..=100.0),
                expected: 50.0,
            },
            Case {
                payload: (27.0, 14.0..=28.0, -100.0..=100.0),
                expected: 86.0,
            },
        ];

        for Case { payload, expected } in CASES {
            let res = map_range(payload.0, payload.1, payload.2);
            eprintln!("Actual: {res}, expected: {expected}");
            assert_approx_eq!(res, expected, 1.0);
        }
    }
}
