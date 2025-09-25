fn main(){
    let res = is_even(32);
    println!("The output is {}", res);
}

fn is_even(num: i32) -> bool {
    if num % 2 == 0 {
        return true;
    }
    return false;
}