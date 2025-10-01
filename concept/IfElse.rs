fn main(){
    // let is_male = true; 
    // let mut is_female = true;

    // if(is_male){
    //     println!("You are eligible for vote");
    // }

    // if(is_female && is_male){
    //     println!("You are eligible for car drive because male value is {} and female value is {}", is_male, is_female);
    // }

    // is_female = false;

    let a_string = "YouAreAGoodStudent";

    // for i in 0..101{
    //     print!("{ }", i );
    // }

    // print!("{}", a_string);

    let mgs: String = String::from("Hello Rust");
    let _char = mgs.chars().nth(0);

    println!("{}",_char.unwrap());

    let _len = mgs.len();
    let s = &mgs;
    println!("Message using reference : {s}");
    println!("Message using reference : {_len}");
    println!("Message using reference : {mgs}");
    println!("Message using reference : {a_string}");

    if false {
        print!("if is true")
    }else if false {
        print!("if else is true")
    }else{
        print!("value if false");
    }
}