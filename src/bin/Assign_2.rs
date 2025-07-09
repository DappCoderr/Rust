fn main(){
    let res = get_the_fibbonacci_series(3);
    println!("The fibbonacci number of giver input is {}", res)
}
// n = 10
// res = 1 -> 2 -> 3 -> 5 -> 8 -> 13 -> 21 -> 34 -> 55
// i = 0 -> 1 -> 1 -> 2 -> 3 -> 5 -> 8 -> 13 -> 21 -> 34
// j= 1 -> 1 -> 2 -> 3 -> 5 -> 8 -> 13 -> 21 -> 34 -> 55
// count = 1 -> 2 -> 3 -> 4 -> 5 -> 6 -> 7 -> 8 -> 9 -> 10
// 0 1 1 2 3 5 8 13 21 34 55

fn get_the_fibbonacci_series(n:i32) -> i32{
    let mut i:i32 = 0;
    let mut j:i32 = 1;
    let mut res: i32 = 0;
    let mut count: i32 = 1;
    for _ in 0..n {
        res = i + j;
        i = j;
        j = res;
        count += 1;
        if count == n{
            return res;
        }
    }
    res
}