fn main(){
    let word = String::from("Your first word");
    let first = get_the_first_word(word);
    println!("The first word is {}", first)
}

fn get_the_first_word(sentence:String) -> String{
    let len = sentence.len();
    for i..len{
        if i == " "{
            break;
        }
    }
}