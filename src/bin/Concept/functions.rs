// use core::num;
// use std::i32;

fn main(){

    // let _sentence = String::from("My name is dc");
    // // let letter = get_letter_from_sentence(_sentence,11);  
    // let _word = get_first_word_from_sentence(_sentence);
    // // println!("The first word is {}", letter);
    // println!("The first word is {}", _word);

    let num_1:u8 = 250;
    let num_2:u8 = 5;

    let result:u8 = add_two_numbers(num_1, num_2);
    println!("sum of both the number {}", result);
}

// fn get_letter_from_sentence(sentence: String, num:usize) -> char {
//     return sentence.chars().nth(num).unwrap();
// }

// fn get_first_word_from_sentence(sentence:String) -> String {
//     let mut ans = String::from("");
//     for chars in sentence.chars(){
//         if chars == ' '{
//             break;
//         }
//         ans.push_str(chars.to_string().as_str());
//     }
//     return ans;
// }

fn add_two_numbers(num_1: u8, num_2:u8) -> u8{
    return num_1+num_2;
}