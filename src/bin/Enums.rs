#![allow(unused)]

#[derive(Debug, PartialEq)]
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

    //particalEq
    println!("{}", Color::Red == Color::Green);
    println!("{}", Color::Red == Color::Red);

    //Option
    let x: Option<i32> = None;
    let x: Option<i32> = Some(-11);

    println!("Option: {:?}", x)

    //Result
    let res: Result<u32,String> = OK(5);
    let res: Result<u32,String> = Err("div by 10".to_string());
    println!("Result: {:?}", res);

}

