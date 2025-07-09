fn main(){
    let given_string = String::from("hnnkshs");
    // let res = find_the_first_a(&given_string);

    match find_the_first_a(&given_string) {
        Some(value) => println!("The first a is at index {}", value),
        None => println!("a not found in the string"),
    }
}

fn find_the_first_a(string: &str) -> Option<i32>{
    for (i,character) in string.chars().enumerate(){
        if character == 'a'{
            return Some(i as i32);
        }
    }
    None
}

