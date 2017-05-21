extern crate genetic_algorithms;
extern crate rand;

use rand::{Rng, XorShiftRng, SeedableRng};
use genetic_algorithms::solution::{PString, Solution};

fn main() {
    let str_set = generate_big_ass_random_set_of_PStrings(50, 6);
    let sol = Solution::new(PString(vec!['h','o','l','i','w','s']), &str_set);
    println!("{}", sol.cost);
}

fn generate_big_ass_random_set_of_PStrings(n: usize, m: usize) -> Vec<PString> {
    let mut v = Vec::with_capacity(n);
    let mut rng : XorShiftRng = SeedableRng::from_seed([1,2,3,4]);
    for _ in 0..n {
        let mut ps = Vec::with_capacity(m);
        for _ in 0..m {
            ps.push(rng.gen_range(b'A', b'Z') as char);
        }
        v.push(PString(ps));
    }
    v

}
