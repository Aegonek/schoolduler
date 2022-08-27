use rand::distributions::Uniform;

use super::*;

// TODO: test
pub fn creep_mutation(chrom: &mut Chromosome, params: &Params, creep_range: &Range<u8>) {
    let distr = Uniform::new(creep_range.start, creep_range.end);
    for lesson in chrom.0.as_raw_mut_slice() {
        let rand = Promile(thread_rng().gen_range(0..1000));
        if rand < params.mutation_probability {
            let creep = distr.sample(&mut thread_rng());
            *lesson = lesson.wrapping_add(creep);
        }
    }
}

// TODO: test
pub fn invert_bit_mutation(chrom: &mut Chromosome, params: &Params) {
    for mut bit in &mut chrom.0 {
        let rand = Promile(thread_rng().gen_range(0..1000));
        if rand < params.mutation_probability {
            let bit_refm = bit.as_mut();
            *bit_refm = !*bit_refm;
        }
    }
}