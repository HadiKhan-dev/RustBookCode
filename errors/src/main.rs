fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

fn main() {


    println!("{:?}",last_char_of_first_line("Hello, world\nHow are you today?"))

    //assert_eq!(last_char_of_first_line(""), None);
    //assert_eq!(last_char_of_first_line("\nhi"), None);
}
