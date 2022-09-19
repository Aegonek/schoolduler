use crate::algen::{Gene, Chromosome};

pub struct Case<T, U>
where
    U: PartialEq,
{
    pub payload: T,
    pub expected: U,
}

#[allow(unused)]
pub fn gene(hour: u16) -> Gene {
    Gene {
        hour,
        teacher: 0,
        student_group: 0
    }
}

pub fn hours(chrom: Chromosome) -> Vec<u16> {
    chrom.0.into_iter().map(|chrom| chrom.hour).collect()
}

macro_rules! chromosome {
    ($($x:expr), *) => {
        crate::algen::Chromosome(vec![$(crate::utils::tests::gene($x), )*])
    };
}

pub(crate) use chromosome;
