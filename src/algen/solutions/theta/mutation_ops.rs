#![allow(irrefutable_let_patterns)]

use super::*;
use rand::prelude::*;

pub fn creep_mutation(solver: &Solution, genes: &mut BitVec<u8>, i: usize) {
    use MutationOp::*;
    let Config { 
        mutation_op: CreepMutation { from_distribution }, .. 
    } = solver.config else { 
        panic!("Tried to invoke creep mutation operator, but algorithm was configured to use another operator.") 
    };
    let byte = i / 8; // TODO: test
    let creep = from_distribution.sample(&mut thread_rng());
    let allel = &mut genes.as_raw_mut_slice()[byte];
    let shifted = allel.wrapping_add(creep);
    *allel = shifted;
}
