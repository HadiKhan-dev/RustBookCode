fn main() {
    let test = "the quick brown fox jumps over the lazy dog";

    let mut fin = String::from("");

    for word in test.split_whitespace() {
        let first_let = &word[0..1];
        let mut new_word = String::new();

        if ["a","e","i","o","u"].contains(&first_let) {
            new_word = String::from(word)+"-hay";
        } else {
            let rest = &word[1..];
            new_word = String::from(rest)+"-"+first_let+"ay";
        }
        println!("The new word is: {}",new_word);
        fin = fin+&new_word+" ";
    }

    println!("{}",fin);
}
