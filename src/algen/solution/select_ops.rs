use super::*;

// TODO: test
pub fn roulette_selection<'a>(population: &'a [Rated<Chromosome>]) -> &'a Rated<Chromosome> {
    let sum_of_ratings: u64 = population
        .iter()
        .map(|x| Into::<u64>::into(x.rating.0))
        .sum();
    let mut random: u64 = thread_rng().gen_range(0..sum_of_ratings);
    for rated in population {
        if random < rated.rating.0.into() {
            return rated;
        }
        random = random.saturating_sub(rated.rating.0.into());
    }

    panic!("Roulette selection should return one chromosome!")
}
