extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {


    let mut fah = String::new();

    io::stdin().read_line(&mut fah).expect("error to std in");

   let fah :f32 = fah.trim().parse().expect("Invalid Input");

    let Cel = (fah - 32.0 )/ 1.8000;

    println!("화씨 {} -> 섭씨{} ",fah , Cel);

    let Fah = Cel * 1.8000 + 32.0;

    println!("섭씨 {} -> 화씨{} ",Cel , Fah);

}
