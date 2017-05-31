extern crate genetic_algorithms;
extern crate rand;
extern crate config;

use config::{Config, File, FileFormat, Value};
use genetic_algorithms::population::Population;
use genetic_algorithms::normalizer::{normalize, decode};
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File as Fl;
use std::env::args;


fn main() {
    let mut config = Config::new();
    config.merge(File::new("Settings", FileFormat::Toml).required(true)).expect("No Configuration file 'Settings.toml'");
    
    let pop_size: usize = config.get_int("pop_size").expect("No pop_size in Settings") as usize;
    let generations: usize = config.get_int("generations").expect("No generations in Settings") as usize;
    let seeds: Vec<u32> = to_u32_vec(config.get_array("seeds").expect("No seeds in Settings"));

    let args: Vec<String> = args().collect();
    let ref filename = args[1];
    let file = Fl::open(filename).unwrap();
    let buf_reader = BufReader::new(file);
    let mut strings: Vec<String> = Vec::new();

    for line in buf_reader.lines(){
        let l = line.unwrap();
        strings.push(l);
    }
    
    let (normalized, key) = normalize(strings.clone());

    for seed in seeds{
        let mut pop = Population::new_random(pop_size, &normalized, key.clone(), [seed, seed*3, seed*7, seed*11]);

        for i in 0..generations{
            println!("generation: {}", i);
            println!("avg_fitness: {:#?}", pop.avg_fitness());
            println!("best_solution: \n{:#?}", pop.best_solution);
            pop.next_generation();
        }
        let string = decode(&pop.best_solution.pstr, &key);
        println!("best solution overall: \n{:?}", pop.best_solution);
        println!("{}", string);
    }
}

fn to_u32_vec(values: Vec<Value>) -> Vec<u32> {
    let mut v = Vec::with_capacity(values.len());
    for vs in &values {
        v.push(vs.clone().into_int().expect("Error converting value to i64") as u32);
    }
    v
}
