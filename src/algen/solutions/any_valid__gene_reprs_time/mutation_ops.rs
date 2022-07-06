use super::*;
use std::cmp::{max, min};
use std::ops::Range;
use tap::{Conv, TryConv};

pub fn creep_mutation(chromosome: &mut [u32], bounds: &Range<u32>) {
    for allel in chromosome {
        if thread_rng().gen_range(0..1000) < MUTATION_PROBABILITY {
            let creep = MUTATION_CREEP_DISTRIBUTION.sample(&mut thread_rng());
            let mut raw = (*allel).conv::<i64>() + creep as i64;
            raw = max(bounds.start.into(), raw);
            raw = min(raw, bounds.end.into()); // TODO: test this
            *allel = raw.try_conv::<u32>().unwrap();
        }
    }
}
