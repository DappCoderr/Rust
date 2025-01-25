#![allow(unused)]

#[derive(Debug)]

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
    let color = Color::Green;
    let color = Color::Rgba(0,0,255,0.1);
    let color = Color::Hex("#ffff".to_string());
    println!("{:?}", {color});
}