#![allow(unused)]

// fn main() {
//     //Arrays
//     let arr: [u32; 3] = [1, 2, 3];
//     println!("Array: {:?}", arr);

//     let arr: [u32; 10] = [0; 10];
//     println!("Initialize array with value 0: {:?}", arr)
// }

fn main(){
    let mut arr: [&str;3] = ["Hello","Hello","Hello"];
    any_function(arr);
    any_function_1(&mut arr);
    println!("main array = {:?}", {arr});
}

fn any_function(mut arr1:[&str;3]){
    arr1[0] = "NEW";
    println!("new array = {:?}", {arr1})
}

fn any_function_1(arr1: &mut [&str;3]){
    arr1[0] = "LION";
    println!("new 1 array = {:?}", {arr1})
}
