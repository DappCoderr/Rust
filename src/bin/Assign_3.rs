fn main(){
    let my_string = String::from("Hello");
    let res = get_the_length_of_the_string(&my_string);
    println!("The length of the given string is {}", res)
}

fn get_the_length_of_the_string(a: &str) -> usize{
    a.chars().count()
}