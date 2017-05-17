pub newtype PString = Vec<u32>;

#[derive(Debug)]
pub struct PString(Vec<u32>);

impl Pstring {
    pub 
}

#[derive(Debug)]
pub struct Solution<'a> {
    pub pstr: PString,
    pub cost: f64,
    pub str_set: &'a Vec<PString> 
}

impl Solution {

    pub fn new(pstr: PString, set: &Vec<PString>) -> Solution {
        let mut s = Solution {
            pstr: pstr,
            cost: 0.0,
            str_set: set
        }
        s.cost = s.cost();
        s
    }

    fn cost(&self) -> f64 {
        let mut max = 0;
        for i in self.str_set {
            
        }
    }
}
