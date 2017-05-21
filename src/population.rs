extern crate rand;

use self::rand::{Rng, XorShiftRng, SeedableRng};
use solution::{PString, Solution};

pub struct Population {
    pub size_limit: usize,
    pub population: Vec<PString>,
    pub sum_fitness: f64,
}

impl Population {
    pub fn new_random(){

    }
}
