use crate::algen::{Chromosome, Gene, params::Params};

#[test]
fn expected_result_invert_bit() {
    let mut chromosome = chromosome!(5, 24, 127, 50);
    invert_bit_mutation(&mut chromosome, &Params::default());
    let values: Vec<_> = chromosome.0.into_iter().map(|chrom| chrom.hour).collect();
    assert_eq!(values, vec![130, 159, 248, 181]);
}

#[allow(unused)]
fn gene(hour: u16) -> Gene {
    Gene {
        hour,
        teacher: 0,
        student_group: 0
    }
}

macro_rules! chromosome {
    ($($x:expr), *) => {
        Chromosome(vec![$(gene($x), )*])
    };
}

pub(self) use chromosome;

use super::invert_bit_mutation;