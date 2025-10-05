#![allow(unused)]

// #[derive(Debug, PartialEq)]

// -----------------------Enums--------------------------
// pub enum Color{
//     Red,
//     Green,
//     Blue,
//     Rgba(u8, u8, u8, f32)
// }
// fn main(){

// }


// -----------------------String and str--------------------------
// fn main(){
//     println!("{}", hello());
//     let msg = "Rust".to_string();
//     let s = &msg;
//     println!("{}", greet(s));
//     println!("{}", append(msg));
// }

// pub fn hello() -> String {
//     "Hello Rust".to_string()
// }

// pub fn greet(name: &str) -> String {
//     format!("Hello {}", name)
// }

// pub fn append(mut s: String) -> String {
//     s += "!";
//     s
// }

// -----------------------Tuples--------------------------
// fn main(){
//     let tupples = (true, 'a', 3.4);
//     println!("{}", tupples.1);

//     let new_tuples = ((true, 4), (4,5,8.3));
//     println!("{}", new_tuples.1.0);
//     let a = 4;

//     let (a,b,c) = tupples;
//     println!("{}",a);

//     println!("{}",a);
// }

// pub fn first(t: (bool, u32, char)) -> bool {
//     let (a, _, _) = t;
//     a
// }

// -----------------------Array--------------------------
// fn main(){
//     let mut arr = [1,2,3,4];
//     println!("{}", arr[2]);

//     let mut new_array = arr.to_vec();

//     new_array.push(10);

//     println!("{:?}", new_array);
// }

