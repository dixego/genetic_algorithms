extern crate genetic_algorithms;
extern crate rand;

use rand::{Rng, XorShiftRng, SeedableRng};
use genetic_algorithms::solution::{PString, Solution};
use genetic_algorithms::population::Population;
use genetic_algorithms::normalizer::normalize;

fn main() {
    let str_set = generate_big_ass_random_set_of_PStrings(5, 4);
    let mut pop = Population::new_random(100, &str_set, [4,3,2,1]);
    let mut vec = pop.population.clone();
    vec.sort_by(|a,b| a.fitness.partial_cmp(&b.fitness).unwrap());
    normalize(vec!["ABCD".to_string(), "ACBD".to_string(), "AABA".to_string(), "BCBC".to_string()]);
    println!("{:#?}", str_set);
    println!("{:#?}", vec);
    println!("{:?}", pop.avg_fitness());
}

fn generate_big_ass_random_set_of_PStrings(n: usize, m: usize) -> Vec<PString> {
    let mut v = Vec::with_capacity(n);
    let mut rng : XorShiftRng = SeedableRng::from_seed([1,2,3,4]);
    for _ in 0..n {
        let mut ps = Vec::with_capacity(m);
        for _ in 0..m {
            ps.push(rng.gen_range(0,4));
        }
        v.push(PString(ps));
    }
    v

}
