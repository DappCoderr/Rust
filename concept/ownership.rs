#![allow(unused)]

fn main(){
    let s:String = String::from("Hello");
    // let (s1, len) = calculate_length_using_tuple(s);
    // let len = calculate_length_using_clone(s.clone());
    let len = calculate_length_using_reference(&s);

    let mut new_s:String = String::from("Hello");
    append_string(&mut new_s);

    println!("Length of {} is {} and {}", s, len, new_s);
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

// borrow and mutate it.
fn append_string(s: &mut String){
    s.push_str("World");
}