use std::fs;
use rand::Rng;

fn main() {

    let contents = fs::read_to_string("advice")
        .expect("Error reading advice file");
    
    let advice: Vec<&str> = contents.trim().split('%').collect();

    let rng: usize = rand::thread_rng().gen_range(0..advice.len());

    println!("{}", advice[rng]);
}
