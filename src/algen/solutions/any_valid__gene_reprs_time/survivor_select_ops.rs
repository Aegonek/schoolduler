use super::*;

pub fn roulette_selection(population: &Vec<(Chromosome, u32)>) -> &Chromosome {
    let sum_of_ratings: u32 = population.iter().map(|x| x.1).sum();
    let mut random: u32 = thread_rng().gen_range(0..sum_of_ratings);
    for (chromosome, rating) in population {
        if random < *rating {
            return chromosome;
        }
        random = random.saturating_sub(*rating);
    }

    panic!("Roulette selection should return one chromosome!")
}