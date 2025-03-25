#![allow(unused)]

fn main(){
    let s:String = String::from("Hello");
    // let (s1, len) = calculate_length_using_tuple(s);
    // let len = calculate_length_using_clone(s.clone());
    let len = calculate_length_using_reference(&s);

    println!("Length of {} is {}", s, len);
}

// tuples
fn calculate_length_using_tuple(s:String)->(String, usize){
    let length = s.len();
    return (s, length);
}

// clone
fn calculate_length_using_clone(s:String)->usize{
    return s.len();
}

// borrow reference
fn calculate_length_using_reference(s: &String)->usize{
    return s.len();
}
