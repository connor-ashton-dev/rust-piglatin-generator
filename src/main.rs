mod piglatin;

use piglatin::utils;

fn main() {
    let my_text = "hi my name is connor";
    let seperated_words: Vec<&str> = my_text.split_whitespace().collect();
    let phrase = utils::translate(&seperated_words);
    println!("{}", phrase);
}
