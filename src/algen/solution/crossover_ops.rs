#[cfg(test)]
mod tests;

use super::*;

pub fn one_point_crossover(x: Chromosome, y: Chromosome) -> (Chromosome, Chromosome) {
    #[cfg(test)]
    let axis = unsafe { tests::CROSSOVER_AXIS };
    #[cfg(not(test))]
    let axis = thread_rng().gen_range(0..x.0.len());
    let (x1, x2) = x.0.split_at(axis);
    let (y1, y2) = y.0.split_at(axis);
    let mut new_x = Vec::from(x1);
    new_x.extend_from_slice(y2);
    let mut new_y = Vec::from(y1);
    new_y.extend_from_slice(x2);
    (Chromosome(new_x), Chromosome(new_y))
}
