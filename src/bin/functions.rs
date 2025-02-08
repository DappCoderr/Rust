use std::i32;

fn main(){

    let _sentence = String::from("My name is dc");
    // let letter = get_letter_from_sentence(_sentence,11);  
    let _word = get_first_word_from_sentence(_sentence);
    // println!("The first word is {}", letter);
    println!("The first word is {}", _word);
}

fn get_letter_from_sentence(sentence: String, num:usize) -> char {
    return sentence.chars().nth(num).unwrap();
}

fn get_first_word_from_sentence(sentence:String) -> String {
    let mut ans = String::from("");
    for chars in sentence.chars(){
        ans.push_str(chars.to_string().as_str());
        if chars == ' '{
            break;
        }
    }
    return ans;
}