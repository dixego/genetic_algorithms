extern crate genetic_algorithms;
extern crate rand;

use rand::{Rng, XorShiftRng, SeedableRng};
//use genetic_algorithms::solution::{PString, Solution};
use genetic_algorithms::population::Population;
use genetic_algorithms::normalizer::normalize;

fn main() {
    let seed = 30;
    let strings = generate_rangom_strings(10, 10);
    //let strings = vec![
    //    "AABAAAAAA".to_string(),
    //    "ABAAABBAA".to_string(),
    //    "AABBAAABA".to_string(),
    //    "AAAABBBAB".to_string(),
    //    "AAABBBAAA".to_string(),
    //    "ABBABAAAB".to_string()
    //];
    let (normalized, key) = normalize(strings);
    let mut pop = Population::new_random(100, &normalized, key.clone(), [seed, seed*3, seed*7, seed*11]);
    println!("{:#?}", normalized);
    println!("{:#?}", pop.population);
    println!("{:#?}", pop.sum_fitness);
    let sol = pop.roulette_random_solution();
    println!("{:#?}", sol);
}

fn generate_rangom_strings(n: usize, L: usize) -> Vec<String> {
    let mut rng: XorShiftRng = SeedableRng::from_seed([4,3,2,1]);
    let mut vec: Vec<String> = Vec::with_capacity(n);
    
    for _ in 0..n {
        let mut s_vec = Vec::with_capacity(L);
        for _ in 0..L {
            s_vec.push(rng.gen_range(b'A', b'Z') as char);
        }
        vec.push(s_vec.into_iter().collect());
    }
    vec
}


