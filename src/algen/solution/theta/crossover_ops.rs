use super::*;
use tap::Tap;

pub fn one_point_crossover(x: Chromosome, y: Chromosome) -> (Chromosome, Chromosome) {
    let axis = thread_rng().gen_range(0..x.0.len());
    let (x1, x2) = x.0.split_at(axis);
    let (y1, y2) = y.0.split_at(axis);
    let new_x = BitVec::from(x1).tap_mut(|x| x.extend_from_bitslice(y2));
    let new_y = BitVec::from(y1).tap_mut(|y| y.extend_from_bitslice(x2));
    (Chromosome(new_x), Chromosome(new_y))
}