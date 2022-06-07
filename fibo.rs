extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn fibo( num1:i32 )-> i32{

    let num = num1;
    if num == 0
    {
        0
    }
    else if num == 1
    {
        1
    }
    else
    {

        fibo(num-1) +fibo(num-2)
    }

}
fn main() {

//피보나치
let mut input = String::new();

io::stdin().read_line(&mut input).expect("ERROR TO READ");

let input :i32 = input.trim().parse().expect("Invalid input");

println!("fibo {} = {}", input , fibo(input));

}
