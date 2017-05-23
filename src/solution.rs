extern crate rand;

use std::fmt;
use self::rand::{Rng};

#[derive(Clone)]
pub struct PString(pub Vec<char>);

pub type PStringError = String;
pub type PSResult<T> = Result<T, PStringError>;

impl PString {

    pub fn new(string: String) -> PString {
        PString(string.chars().collect())
    }

    pub fn vec(&self) -> &Vec<char> {
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
        let string: String = self.0.iter().cloned().collect();
        write!(f, "{}", string)
    }
}

#[derive(Clone)]
pub struct Solution<'a>{
    pub pstr: PString,
    pub fitness: f64,
    pub str_set: &'a Vec<PString>,
}

impl<'a> Solution<'a> {

    pub fn new(pstr: PString, set: &Vec<PString>) -> Solution {
        let mut s = Solution {
            pstr: pstr,
            fitness: 0.0,
            str_set: set
        };
        s.fitness = s.fitness();
        s
    }

    pub fn fitness(&self) -> f64 {
        let mut max = 0;
        for s in self.str_set {
            let dist = self.pstr.distance(&s).unwrap();
            if dist > max {
                max = dist
            }
        }
        2.0 - (max as f64)/(self.pstr.len() as f64)
    }

    pub fn recombine_random<T: Rng>(&self, other: &Solution, rng: &mut T) -> Solution {
        let j = rng.gen_range(1, self.pstr.len() - 1); 
        self.recombine_fixed(other, j)
    }

    pub fn recombine_fixed(&self, other: &Solution, j: usize) -> Solution {
        let len = self.pstr.len();
        let mut v = Vec::with_capacity(len);
        let vec1 = self.pstr.vec();
        let vec2 = other.pstr.vec();

        for i in 0..(j+1) {
            v.push(vec1[i]);
        }

        for i in (j+1)..len {
            v.push(vec2[i]);
        }

        Solution::new(PString(v), self.str_set)
        
    }

    pub fn mutate<T: Rng>(&self, rng: T, probability: f64) {}
}

impl<'a> fmt::Debug for Solution<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Solution {{ pstr: {:?},  fitness: {:?} }}", self.pstr, self.fitness)
    }
}
