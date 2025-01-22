#![allow(unused)]

fn main() {
    let mgs: String = String::from("Hello Rust");
    let len: usize = mgs.len();
    let s: &str = &mgs;
    println!("Message using reference : {s}");

    println!("Message : {mgs}");
    println!("Message Length : {len}");

    //string slice
    let s: &str = &mgs[0..2];
    println!("Sliced Message : {s}");

    let len: usize = s.len();
    println!("length of s: {len}");

    let hello: &str = "Hello";
    println!("{hello}");

    let s: &str = r#"
    {
        "a":1,
        "b":{"c":2},
        "c":3
    }
    "#;
    println!("{s}");

    let mut s: String = "Hello String".to_string();
    s += "!";
    println!("{s}");

    //format in rust
    let hello = "rust";
    let emo = "emoji";
    let msg = format!("Hello {hello} {emo}");

    println!("{msg}");
}
