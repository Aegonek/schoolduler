use super::*;

// TODO: test
pub fn roulette_selection(population: &[Rated<Chromosome>]) -> Rated<Chromosome> {
    let sum_of_ratings: u32 = population.iter().map(|x| x.rating).sum();
    let mut random: u32 = thread_rng().gen_range(0..sum_of_ratings);
    for rated in population {
        if random < rated.rating {
            return rated.clone();
        }
        random = random.saturating_sub(rated.rating);
    }

    panic!("Roulette selection should return one chromosome!")
}