fn main() {
    let mut first = String::from("first");
    let mut apple = String::from("apple");
    first = convert(&first);
    apple = convert(&apple);
    println!("First is now: {}", first);
    println!("Apple is now: {}", apple);
    parse_sentence("This is a sentence");
}

fn parse_sentence(string: &str) {
    for word in string.split(" ") {
        print!("{} ", convert(word));
    }
    println!();
}

fn convert(string: &str) -> String {
    let mut new_string = String::new();
    let first_char: char = string.chars().next().unwrap();
    if !is_vowel(first_char) {
        for (i, character) in string.chars().enumerate() {
            if i == 0 {
                continue
            }
            new_string.push(character);
        }
        new_string.push('-');
        new_string.push(first_char);
        new_string.push_str("ay");
    } else {
        new_string.push_str(string);
        new_string.push_str("-hay");
    }
    new_string
}

fn is_vowel(character: char) -> bool {
    match character {
        'a' => true,
        'e' => true,
        'i' => true,
        'o' => true,
        'u' => true,
        'y' => true,
        _ => false,
    }
}