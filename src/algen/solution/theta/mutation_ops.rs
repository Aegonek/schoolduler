use super::*;

pub fn creep_mutation(creep_distribution: Uniform<u8>, genes: &mut BitVec<u8>, i: usize) {
    let byte = i / 8; // TODO: test
    let creep = creep_distribution.sample(&mut thread_rng());
    let allel = &mut genes.as_raw_mut_slice()[byte];
    let shifted = allel.wrapping_add(creep);
    *allel = shifted;
}

// TODO: test
pub fn invert_bit_mutation(genes: &mut BitVec<u8>, i: usize) {
    let mut bit_proxy = genes.get_mut(i).unwrap();
    let bit_refm = bit_proxy.as_mut();
    *bit_refm = !*bit_refm;
}