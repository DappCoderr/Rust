#![allow(unused)]

fn main(){
    // defining the tuple
    let t : (bool, i32, char) = (true, 45, 'a');

    // destructuring the tuple
    let(a, b, c) = t;

    println!("Print the value of a: {}", t.0);
    println!("Print the value of b: {}", t.1);
    println!("Print the value of b: {}", t.2);

    // ignore with _

    let (_, b, _) = t;

    // empty tuple

    let t = ();

    // nested tuple
    let nested = ((1.23, 'a'), (true, 1u32, 'b'), ());
}