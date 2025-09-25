fn main(){
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    vec.push(4);
    vec.push(5);

    println!("The event vector is {:?}", get_even_vector(vec));
}

fn get_even_vector(old_vec: Vec<i32>) -> Vec<i32>{
    let mut new_vec = Vec::new();
    for val in old_vec{
        if val%2 == 0{
            new_vec.push(val);
        }
    }
    return new_vec;
}