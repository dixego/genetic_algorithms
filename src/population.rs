extern crate rand;

use self::rand::{Rng, XorShiftRng, SeedableRng};
use solution::{PString, Solution};
use normalizer::{ReverseKey};

pub struct Population<'a> {
    pub size_limit: usize,
    pub str_set: &'a Vec<PString>,
    pub population: Vec<Solution>,
    pub best_solution: Solution,
    pub sum_fitness: f64,
    pub rng: XorShiftRng,
    pub key: ReverseKey
}

impl<'a> Population<'a> {

    pub fn new_from_init(limit: usize, str_set: &'a Vec<PString>, key: ReverseKey, seed: [u32; 4]) -> Population<'a> {
        let mut rng = SeedableRng::from_seed(seed);
        let mut vec = Vec::with_capacity(limit);
        let mut sum_fitness = 0.0;
        let fake_best = Solution::new(PString(vec![10000; str_set[0].len()]), str_set);
        let mut pop = Population {
            size_limit: limit,
            str_set: str_set,
            population: vec![],
            best_solution: fake_best,
            sum_fitness: 0.0,
            rng:rng,
            key: key
        };

        for i in 0..limit {
            let solution = Solution::new(str_set[i%str_set.len()].clone(), &str_set);
            sum_fitness = solution.fitness;
            if solution.fitness >= pop.best_solution.fitness{
                pop.best_solution = solution.clone();
            }
            vec.push(solution);
        }
        pop.population = vec;
        pop.sum_fitness = sum_fitness;
        pop
    }

    pub fn new_random(limit: usize, str_set: &'a Vec<PString>, key: ReverseKey, seed: [u32; 4]) -> Population<'a> {
        let mut rng = SeedableRng::from_seed(seed);
        let mut vec = Vec::with_capacity(limit);
        let mut sum_fitness = 0.0;
        let fake_best = Solution::new(PString(vec![10000; str_set[0].len()]), str_set);
        println!("FAKE_BEST: {:?}", fake_best);
        
        let mut pop = Population {
            size_limit: limit,
            str_set: str_set,
            population: vec![],
            best_solution: fake_best,
            sum_fitness: 0.0,
            rng:rng,
            key: key
        };

        for _ in 0..limit{
            let solution = pop.generate_random_solution();
            sum_fitness += solution.fitness;
            if solution.fitness >= pop.best_solution.fitness {
                pop.best_solution = solution.clone();
            }
            vec.push(solution);
        }

        pop.population = vec;
        pop.sum_fitness = sum_fitness;
        pop
    }

    pub fn next_generation(&mut self) {
        let mut new_pop = Vec::with_capacity(self.size_limit);
        let mut count = 0;
        let mut new_sum = 0.0;
        let mut best = self.best_solution.clone();

        while count < self.size_limit {
            let mut i = self.roulette_random_solution();
            let mut j = self.roulette_random_solution();
            if i.pstr.vec() == j.pstr.vec() {
                continue;
            }
            let mut sons = i.recombine_random(&j, &mut self.rng, self.str_set);
            let mut ij = sons.0;
            let mut ji = sons.1;
            ij.mutate(0.85, &self.key, self.str_set, &mut self.rng);
            ji.mutate(0.85, &self.key, self.str_set, &mut self.rng);
            new_sum += ij.fitness + ji.fitness;
            if ij.fitness > best.fitness {
                best = ij.clone()
            }
            if ji.fitness > best.fitness {
                best = ji.clone()
            }
            count += 2;
            new_pop.push(ij);
            new_pop.push(ji);
        }

        self.population = new_pop;
        self.sum_fitness = new_sum;
        self.best_solution = best;
    }

    pub fn best_solution(&self) -> &Solution {
        let mut max = &self.population[0];
        for sol in &self.population {
            if sol.fitness > max.fitness {
                max = &sol;
            }
        }
        max
    }

    pub fn generate_random_solution(&mut self) -> Solution {
        let len = self.str_set[0].len();
        let mut vec = Vec::with_capacity(len);

        for i in 0..len {
            vec.push(self.rng.gen_range(0, self.key[i].len()) as u32);
        }
        Solution::new(PString(vec), self.str_set)
    }

    pub fn roulette_random_solution(&mut self) -> Solution {
        let mut upper = 1.0;
        let mut prob: f64 = self.rng.gen_range(0.0, upper);
        let mut sol: Option<&Solution> = None;
        let mut i = self.rng.gen_range(0, self.population.len());
        let mut count = 0;

        // pa' que no se atore
        while sol.is_none() {
            if (upper) < 0.001{
                sol = Some(&self.population[i]);
                break;
            }
            if count >= 10 {
                count = 0;
                upper *= upper * 0.75;
                prob = self.rng.gen_range(0.0, upper);
            }
            if prob <= self.population[i].fitness {
                sol = Some(&self.population[i]);
            }else{
                i = self.rng.gen_range(0, self.population.len());
                count += 1;
            }
        }
        
        sol.unwrap().clone()
    }

    pub fn avg_fitness(&self) -> f64 {
        self.sum_fitness/(self.population.len() as f64)
    }

    fn roulette_random(&self) -> &Solution {
        &self.population[0]
    }
}
