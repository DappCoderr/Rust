#![allow(unused)]

pub fn mul(a:u32, b:u32) -> u32{
    a*b
}

pub fn div(a:u32, b:u32) -> u32{
    a/b
}

fn main() {
    let res1:u32 = mul(2,3);
    let res2:u32 = div(10,5);
    println!("Mul is: {}", res1);
    println!("Div is: {}", res2);
}
