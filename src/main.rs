use rand::{thread_rng, Rng};
use std::fs::File;
use std::io::Read;

fn gen_affix(symbols: &Vec<char>, amount: i32) -> String {
    let mut affix="".to_string();
    let sep = &symbols[rand::thread_rng().gen_range(0..symbols.len())].to_string();
    for _i in 0..amount {
        affix.push_str(sep);
    }
    affix
}


fn gen_word(words: &Vec<&str>, uppercase_probability: i32) -> String {
    let word = words[thread_rng().gen_range(0..words.len())];

    if thread_rng().gen_range(0..=99) < uppercase_probability {
        word.to_uppercase()
    } else {
        word.to_string()
    }



}
// Get references instead of the actual object to prevent moving.
fn gen_words(words: &Vec<&str>,separator: &String, amount: i32, uppercase_probability: i32) -> String {
    let mut word_str = "".to_string();
    for _i in 0..amount {
        word_str.push_str(&gen_word(words, uppercase_probability));
        word_str.push_str(&separator);
    }

    word_str

}

fn main() {

    // Simplest implementation. Likely to be possible to do simpler,
    // but I'm unaware what approach would be better; Vectors or Arrays or Slices here lmao.
    let valid_symbols: Vec<char> = "§\"#¤%&/()=?`,.-'<>;:_@£$€{[]}|~^¨*\\µ´+½".chars().collect();
    let num_words = 4;
    let wordlist = "wordlists/words.txt";
    let mut file = File::open(&wordlist).expect("Can't open file");
    let mut text = String::new();
    file.read_to_string(&mut text).expect("Can't read file");
    let words = text.lines().collect::<Vec<_>>();

    let mut password = "".to_string();
    let init_sep = gen_affix(&valid_symbols, 2);
    password.push_str(&init_sep);
    password.push_str(&thread_rng().gen_range(10..=99).to_string()); // Fulfill
    let separator = gen_affix(&valid_symbols, 1);
    password.push_str(&separator.to_string());
    password.push_str(&gen_words(&words,&separator,num_words,50));
    password.push_str(&thread_rng().gen_range(10..=99).to_string());
    password.push_str(&init_sep);

    println!("{}", &password);
}
