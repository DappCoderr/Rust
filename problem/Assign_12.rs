fn main(){
    let word = String::from("Hello World!!");
    let new_word = find_first_word(&word);
    println!("{}", new_word);
}

fn find_first_word(s: &str) -> &str{
    let mut idx = 0;
    for i in s.chars(){
        if i == ' '{
            break;
        }
        idx = idx + 1;
    }
    return &s[0..idx];
}