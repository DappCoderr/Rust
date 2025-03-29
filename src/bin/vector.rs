#[allow(unused)]

// fn main(){
//     let mut v = Vec::<i32>::new();
//     let mut vs = vec![6,7,8,9];

//     v.push(1);
//     v.push(2);
//     v.push(3);
//     v.push(4);
//     v.pop();
//     println!("Vector = {:?}",{v.clone()});
//     vs.push(10);
//     v.pop();
//     println!("Vector = {:?}",{vs});
//     println!("Vector = {:?}",{v});
// }

fn main(){
    let v = vec!["Hello","Hello", "Todo"];
    another_fn(v.clone());
    println!("Print vector {:?}", v);
}

fn another_fn(v1:Vec<&str>){
    println!("Print another vector {:?}", v1);
}