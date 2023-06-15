mod euler;

use euler as eu;

fn main() {
    let rc = eu::p001(1000);
    println!("Problem 001 = {}", rc);
    println!("Problem 002 = {}", eu::p002(4000_000));
    println!("Problem 003 = {}", eu::p003(600851475143));
    println!("Problem 004 = {}", eu::p004());
    println!("Problem 005 = {}", eu::p005());
    println!("Problem 006 = {}", eu::p006(100));
    println!("Problem 007 = {}", eu::p007(10001));
    println!("Problem 008 = {}", eu::p008());
    println!("Problem 009 = {}", eu::p009());
    println!("Problem 010 = {}", eu::p010());
}