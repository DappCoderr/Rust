#![allow(unused)]

enum Color{
    Red,
    Blue,
    Yellow,
    Green,
    Rgba(u8,u8,u8,f32),
    Hex(String),
    Hsl{h:u8,s:u8,l:u8}
}

fn main(){
    let color : Color = Color::Red;
}