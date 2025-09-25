fn main(){
    let num = vec![1,2,3];
    let num_iter = num.iter();
    let filter_num = num_iter.filter(|x| *x%2 == 0);
    for var in filter_num{
        println!("{}", var);
    }
}