use std::mem;

use rand::distributions::Uniform;

use super::*;

// TODO: test
pub fn creep_mutation(chrom: &mut Chromosome, params: &Params, creep_range: &Range<u16>) {
    let distr = Uniform::new(creep_range.start, creep_range.end);
    for hour in chrom.0.iter_mut().map(|gene| &mut gene.hour) {
        let rand = Promile(thread_rng().gen_range(0..1000));
        if rand < params.mutation_probability {
            let creep = distr.sample(&mut thread_rng());
            *hour = hour.wrapping_add(creep);
        }
    }
}

// TODO: test
#[allow(dead_code)]
pub fn invert_bit_mutation(chrom: &mut Chromosome, params: &Params) {
    for hour in chrom.0.iter_mut().map(|gene| &mut gene.hour) {
        let mut shift: u16 = 0;
        for i in 0..(mem::size_of::<u16>() as u32 * 8) {
            let rand = thread_rng().gen_range(0..1000);
            if rand < params.mutation_probability.promiles() {
                shift += 2_u16.pow(i);
            }
        }
        *hour ^= shift;
    }
}
