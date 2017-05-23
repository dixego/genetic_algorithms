extern crate rand;

use self::rand::{Rng, XorShiftRng, SeedableRng};
use solution::{PString, Solution};

pub struct Population<'a> {
    pub size_limit: usize,
    pub str_set: &'a Vec<PString>,
    pub population: Vec<Solution<'a>>,
    pub sum_fitness: f64,
    pub rng: XorShiftRng
}

impl<'a> Population<'a> {
    pub fn new_random(limit: usize, str_set: &'a Vec<PString>, seed: [u32; 4]) -> Population<'a> {
        let mut rng = SeedableRng::from_seed(seed);
        let mut vec = Vec::with_capacity(limit);
        let len = str_set[0].len();
        let mut sum_fitness = 0.0;

        for _ in 0..limit{
            let solution = Population::generate_random_solution(len, &str_set, &mut rng);
            sum_fitness += solution.fitness;
            vec.push(Population::generate_random_solution(len, &str_set, &mut rng));
        }

        Population {
            size_limit: limit,
            str_set: str_set,
            population: vec,
            sum_fitness: sum_fitness,
            rng:rng
        }
    }

    fn generate_random_solution<T: Rng>(len: usize, set: &'a Vec<PString>, rng: &mut T) -> Solution<'a> {
        let mut vec = Vec::with_capacity(len);
        for _ in 0..len {
            vec.push(rng.gen_range(b'A', b'E') as char);
        }
        Solution::new(PString(vec), set)
    }

    pub fn avg_fitness(&self) -> f64 {
        self.sum_fitness/(self.population.len() as f64)
    }

    fn roulette_random(&self) -> &Solution {
        &self.population[0]
    }
}
