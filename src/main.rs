use rand::Rng;

fn main() {

    let bytes = include_bytes!("advice");
    let contents = String::from_utf8_lossy(bytes);
    
    let advice: Vec<&str> = contents.trim().split('%').collect();

    let rng: usize = rand::thread_rng().gen_range(0..advice.len());

    println!("{}", advice[rng]);
}
