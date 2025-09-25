fn main(){
    let vec = vec![2,4,6,7,5,1,3];
    let res = filter_and_map(vec);
    println!("{:?}", res);
}

fn filter_and_map(v: Vec<i32>) -> Vec<i32>{
    let new_itera = v.iter().filter(|x| *x%2==0).map(|x| x*2);
    let vec = new_itera.collect();
    return vec;
}