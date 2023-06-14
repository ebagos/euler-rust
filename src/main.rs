mod euler;

use euler::{p001, p002, p003};

fn main() {
    let rc = p001(1000);
    println!("Problem 001 = {}", rc);
    println!("Problem 002 = {}", p002(4000_000));
    println!("Problem 003 = {}", p003(600851475143));
}