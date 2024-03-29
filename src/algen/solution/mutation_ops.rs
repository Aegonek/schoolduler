#[cfg(test)]
mod tests;

use once_cell::unsync::Lazy;
use rand::distributions::Uniform;

use super::*;

#[allow(dead_code)]
pub fn creep_mutation(chrom: &mut Chromosome) {
    const CONFIG: Config = config::CONFIG;
    const DELTA_RANGE: Range<i16> = -20..20;
    thread_local! { static DELTA_DISTR: Lazy<Uniform<i16>> = Lazy::new(|| Uniform::new(DELTA_RANGE.start, DELTA_RANGE.end)); }
    for hour in chrom.0.iter_mut().map(|gene| &mut gene.hour) {
        let hour: &mut u16 = hour;
        #[cfg(test)]
        let rand: u32 = 0; // always mutate
        #[cfg(not(test))]
        let rand: u32 = thread_rng().gen_range(0..1000);
        if rand < CONFIG.mutation_probability.promiles() {
            #[cfg(test)]
            let creep = unsafe { tests::CREEP };
            #[cfg(not(test))]
            let creep = DELTA_DISTR.with(|distr| distr.sample(&mut thread_rng()));
            *hour = hour.saturating_add_signed(creep);
        }
    }
}

#[allow(dead_code)]
pub fn invert_bit_mutation(chrom: &mut Chromosome) {
    const CONFIG: Config = config::CONFIG;
    for hour in chrom.0.iter_mut().map(|gene| &mut gene.hour) {
        let hour: &mut u16 = hour;
        let mut shift: u16 = 0;
        // Making function use fixed values for tests.
        #[cfg(test)]
        let mut bits: Vec<u32> = vec![
            1000, 1000, 1000, 1000, 1000, 1000, 1000, 1000, 0, 1000, 1000, 1000, 1000, 0, 0, 0,
        ]; // mutate 0th, 1st, 2nd and 8th bit
        for i in 0..16 {
            #[cfg(test)]
            let rand = bits.pop().unwrap();
            #[cfg(not(test))]
            let rand = thread_rng().gen_range(0..1000);
            if rand < CONFIG.mutation_probability.promiles() {
                shift |= 2_u16.pow(i);
            }
        }
        *hour ^= shift;
    }
}
