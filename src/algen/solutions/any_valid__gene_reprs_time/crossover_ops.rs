use super::*;
use tap::Tap;

pub fn one_point_crossover(x: Chromosome, y: Chromosome) -> (Chromosome, Chromosome) {
    let axis = thread_rng().gen_range(0..x.len());
    let (x1, x2) = x.split_at(axis);
    let (y1, y2) = y.split_at(axis);
    let new_x = Vec::from(x1).tap_mut(|x| x.extend_from_slice(y2));
    let new_y = Vec::from(y1).tap_mut(|y| y.extend_from_slice(x2));
    (new_x, new_y)
}