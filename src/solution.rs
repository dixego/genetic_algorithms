extern crate rand;

use std::fmt;
use self::rand::{Rng};
use normalizer::ReverseKey;

#[derive(Clone)]
pub struct PString(pub Vec<u32>);

pub type PStringError = String;
pub type PSResult<T> = Result<T, PStringError>;

impl PString {

    pub fn new(vec: Vec<u32>) -> PString {
        PString(vec)
    }

    pub fn vec(&self) -> &Vec<u32> {
        &self.0
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn distance(&self, other: &PString) -> PSResult<u32> { 
        let mut count = 0;
        let vec = self.vec();
        let o_vec = other.vec();

        if vec.len() != o_vec.len() {
            return Err("PStrings have different length, cannot calculate distance.".to_string());
        }

        for i in 0..(o_vec.len()) {
            if vec[i] != o_vec[i] {
                count += 1;
            }
        }
        Ok(count)
    }
}

impl fmt::Debug for PString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

#[derive(Clone)]
pub struct Solution{
    pub pstr: PString,
    pub fitness: f64,
    pub distance: u32
}

impl Solution {

    pub fn new(pstr: PString, set: &Vec<PString>) -> Solution {
        let mut s = Solution {
            pstr: pstr,
            fitness: 0.0,
            distance: 0,
        };
        let x = s.fitness(set);
        s.fitness = x.0;
        s.distance = x.1;
        s
    }

    pub fn fitness(&self, set: &Vec<PString>) -> (f64, u32) {
        let mut max = 0;
        for s in set {
            let dist = self.pstr.distance(&s).unwrap();
            if dist > max {
                max = dist
            }
        }
        (1.0 - (max as f64)/(self.pstr.len() as f64), max)
    }

    pub fn recombine_random<T: Rng>(&self, other: &Solution, rng: &mut T, set: &Vec<PString>) -> (Solution, Solution) {
        let j = rng.gen_range(1, self.pstr.len() - 1); 
        self.recombine_fixed(other, j, set)
    }

    pub fn recombine_fixed(&self, other: &Solution, j: usize, set: &Vec<PString>) -> (Solution, Solution) {
        let len = self.pstr.len();
        let mut v1 = Vec::with_capacity(len);
        let mut v2 = Vec::with_capacity(len);
        let vec1 = self.pstr.vec();
        let vec2 = other.pstr.vec();

        for i in 0..(j+1) {
            v1.push(vec1[i]);
            v2.push(vec2[i]);
        }

        for i in (j+1)..len {
            v1.push(vec2[i]);
            v2.push(vec1[i]);
        }

        (Solution::new(PString(v1), set), Solution::new(PString(v2), set))
        
    }

    pub fn mutate<T: Rng>(&mut self, probability: f64, key: &ReverseKey, set: &Vec<PString>, rng: &mut T) {
        let p = rng.gen_range::<f64>(0.0, 1.0);
        if p <= probability {
            let i = rng.gen_range(0, self.pstr.vec().len());
            self.pstr.0[i] = rng.gen_range(0, key[i].len()) as u32;
            let x = self.fitness(set);
            self.fitness = x.0;
            self.distance= x.1;
        }
    }
}

impl fmt::Debug for Solution {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Solution {{ pstr: {:?}, \tfitness: {:?}, \tdistance: {:?}}}", self.pstr, self.fitness, self.distance)
    }
}
