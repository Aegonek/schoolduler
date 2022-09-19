#[cfg(test)]
mod tests;

use super::*;

pub fn roulette_selection<'a, T: Borrow<Chromosome> + 'a>(population: &'a [Rated<T>]) -> &'a Rated<T> {
    let sum_of_ratings: u64 = population
        .iter()
        .map(|x| Into::<u64>::into(x.rating.value()))
        .sum();
    let mut random: u64 = thread_rng().gen_range(0..sum_of_ratings);
    for rated in population {
        if random < rated.rating.value().into() {
            return rated;
        }
        random = random.saturating_sub(rated.rating.value().into());
    }

    panic!("Roulette selection should return one chromosome!")
}
