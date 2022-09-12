use bitvec::prelude::*;
use bitvec::view::BitView;
use rand::distributions::Uniform;

use super::*;

// TODO: test
pub fn creep_mutation(chrom: &mut Chromosome, params: &Params, creep_range: &Range<u8>) {
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
pub fn invert_bit_mutation(chrom: &mut Chromosome, params: &Params) {
    for hour in chrom.0.iter_mut().map(|gene| &mut gene.hour) {
        for mut bit in hour.view_bits_mut::<Msb0>() {
            let rand = Promile(thread_rng().gen_range(0..1000));
            if rand < params.mutation_probability {
                let bit_refm = bit.as_mut();
                *bit_refm = !*bit_refm;
            }
        }
    }
}
