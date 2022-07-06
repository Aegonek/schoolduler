
use super::*;
use crate::utils::testing::Case;

const GET_BIT_RETURNS_CORRECT_VALUE_CASES: [Case<(u32, usize), bool>; 4] = [
    Case { payload: (240, 0), expected: false },
    Case { payload: (253, 0), expected: true },
    Case { payload: (1088, 6), expected: true },
    Case { payload: (4159, 6), expected: false },
];

#[test]
fn get_bit_returns_correct_value() {
    for Case { payload: (number, index), expected } in GET_BIT_RETURNS_CORRECT_VALUE_CASES {
        let res = number.get_bit(index);
        eprintln!("Number: {0} - {0:#018b}. We expect that {1} bit has value: {2}. Actual value: {3}", number, index + 1, expected, res);
        assert!(res == expected)
    }
}

const SET_BIT_SETS_CORRECT_VALUE_CASES: [Case<(u32, usize, bool), u32>; 4] = [
    Case { payload: (240, 0, true), expected: 241 },
    Case { payload: (253, 1, true), expected: 255 },
    Case { payload: (1088, 6, false), expected: 1024 },
    Case { payload: (4159, 2, false), expected: 4155 },
];

#[test]
fn set_bit_sets_correct_value() {
    for Case { payload: (mut number, index, value), expected } in SET_BIT_SETS_CORRECT_VALUE_CASES {
        eprintln!("Number: {0} - {0:#018b}. We expect that after setting {1} bit to {2} value will be: {3}", number, index + 1, value, expected);
        number.set_bit(index, value);
        eprintln!("Actual value: {0} - {0:#018b}.", number);
        assert!(number == expected);
    }
}

#[test]
fn modify_bit_commits_change() {
    let mut number: u8 = 0;
    number.modify_bit(0, |bit| *bit = !*bit);
    assert!(number == 1);
}

#[test]
#[should_panic]
fn accessing_out_of_bounds_bit_panics() {
    let number: u8 = 0;
    number.get_bit(8);
}