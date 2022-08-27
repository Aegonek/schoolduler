use std::ops::RangeInclusive;
use num::Num;

// Remap a number within range `old` to a number in range `new`
// Example - map_range(5, 0..=10, -10..=10) == 0
pub fn map_range<Number: Num + Copy>(nmb: Number, old: RangeInclusive<Number>, new: RangeInclusive<Number>) -> Number {
    let ratio = (*new.end() - *new.start()) / (*old.end() - *old.start());
    let new = (nmb - *old.start()) * ratio + *new.start();
    new
}

#[cfg(test)]
mod tests {
    use crate::utils::tests::Case;

    use super::*;

    #[test]
    fn map_range_correct() {
        for Case { payload, expected } in CASES__MAP_RANGE_CORRECT {
            let res = map_range(payload.0, payload.1, payload.2);
            assert_eq!(res, expected);
        }
    }

    const CASES__MAP_RANGE_CORRECT: [Case<(i32, RangeInclusive<i32>, RangeInclusive<i32>), i32>; 3] = [
        Case { payload: (5, 0..=10, -10..=10), expected: 0 }, 
        Case { payload: (0, 0..=10, 50..=100), expected: 50 }, 
        Case { payload: (27, 14..=28, -100..=100), expected: 82 }, 
    ];
}