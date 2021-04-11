
use clap::{App};
use rust_playground::*;
fn main() {
    let matches = App::new("My Super Program")
        .about("Does awesome things")
        .arg("<seq>... 'A sequence of whole positive numbers, i.e. 20 25 30'")
        .get_matches();

    println!("{:?}",matches);

    let mut vect: Vec<u32> = vec![];
    for v in matches
        .values_of_t::<u32>("seq")
        .unwrap_or_else(|e| e.exit())
    {
        vect.push(v);
    }
    let a = vect[0];
    let b = vect[1];
    println!("distance: {}\nadded: {}", distance((a as f32, b as f32),(b as f32, a as f32)), add_int(&a,&b));
}
