use super::*;

// TODO: test
pub fn roulette_selection(population: &[Rated<Chromosome>]) -> Rated<Chromosome> {
    let sum_of_ratings: u64 = population.iter().map(|x| x.rating as u64).sum();
    let mut random: u64 = thread_rng().gen_range(0..sum_of_ratings);
    for rated in population {
        if random < rated.rating as u64 {
            return rated.clone();
        }
        random = random.saturating_sub(rated.rating as u64);
    }

    panic!("Roulette selection should return one chromosome!")
}