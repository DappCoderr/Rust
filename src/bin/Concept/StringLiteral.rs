#![allow(unused)]

fn main(){
    let mut string_literal: String = String::from("Hello world");
    string_literal.push_str("!!!");
    println!("{}", string_literal);

    let s: &str = "Hello world";
    println!("{}",s);
}