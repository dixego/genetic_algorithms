
#[derive(Debug)]
pub struct PString(pub Vec<u32>);

impl PString {
}

#[derive(Debug)]
pub struct Solution<'a> {
    pub pstr: PString,
    pub cost: f64,
    pub str_set: &'a Vec<PString> 
}

impl<'a> Solution<'a> {

    pub fn new(pstr: PString, set: &Vec<PString>) -> Solution {
        let mut s = Solution {
            pstr: pstr,
            cost: 0.0,
            str_set: set
        };
        s.cost = s.cost();
        s
    }

    fn cost(&self) -> f64 {
        let mut max = 0;
        for i in self.str_set {
            
        }
        0.0
    }
}
