use crate::algen::{Chromosome, Gene, config::Config};

#[test]
fn expected_results_invert_bit() {
    let mut chromosome = chromosome!(5, 24, 127, 50);
    invert_bit_mutation(&mut chromosome, &Config::default());
    let hours = hours(chromosome);
    assert_eq!(hours, vec![130, 159, 248, 181]);
}

#[test]
fn expected_results_creep_mutation() {
    {
        let mut chromosome = chromosome!(5, 24, 127, 50, u16::MIN, u16::MAX);
        let creep_range = -20..20;
        creep_mutation(&mut chromosome, &Config::default(), creep_range);
        let hours = hours(chromosome);
        assert_eq!(hours, vec![25, 44, 147, 70, u16::MIN + 20, u16::MAX])
    }
    {
        let mut chromosome = chromosome!(5, 24, 127, 50, u16::MIN, u16::MAX);
        let creep_range = -40..-20;
        creep_mutation(&mut chromosome, &Config::default(), creep_range);
        let hours = hours(chromosome);
        assert_eq!(hours, vec![0, 4, 107, 30, u16::MIN, u16::MAX - 20])
    }
}

#[allow(unused)]
fn gene(hour: u16) -> Gene {
    Gene {
        hour,
        teacher: 0,
        student_group: 0
    }
}

fn hours(chrom: Chromosome) -> Vec<u16> {
    chrom.0.into_iter().map(|chrom| chrom.hour).collect()
}

macro_rules! chromosome {
    ($($x:expr), *) => {
        Chromosome(vec![$(gene($x), )*])
    };
}

pub(self) use chromosome;

use super::{invert_bit_mutation, creep_mutation};