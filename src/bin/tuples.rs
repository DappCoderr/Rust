#![allow(unused)]

fn main(){
    // defining the tuple
    let t : (bool, i32, char) = (true, 45, 'a');

    // destructuring the tuple
    let(a, b, c) = t;

    println!("Print the value of a: {a}");
    println!("Print the value of b: {b}");
    println!("Print the value of b: {c}");
}