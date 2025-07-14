fn main(){
    let larger_num = largest(2,1);
    let larger_char = largest("hello","world");

    println!("value of first fn is {} and second fn is {}", larger_num, larger_char);
}

fn largest <T: std::cmp::PartialOrd>(a:T, b:T) -> T{
    if a > b{
        a
    } else {
        b
    }
}