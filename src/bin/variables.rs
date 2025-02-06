#![allow(unused)]

fn main() {
    let x = 12;
    // x += 1; This will throw an error, Will not compile.
    println!("value of x is: {x}");

    let mut y = 20;
    y += 2;
    println!("value of y is: {y}");

    const h: i32 = 5; // For constant you have to define the type

    let y = true;

    // let v : Vec<{integer}> = vec![1,2,3];

    // println!("Value of this vector: {v}");
}
