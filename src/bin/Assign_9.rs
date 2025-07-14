fn main(){
    let num = vec![1,2,3];
    let num_iter = num.iter();
    let map_num = num_iter.map(|x| x+1);
    println!("new vector is {:?}", map_num);
    for var in map_num{
        println!("{}", var);
    }
    println!("old vector is {:?}", num);
}