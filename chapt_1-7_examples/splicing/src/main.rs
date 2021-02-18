fn main() {
    let s = "hello world";
    let s3 = String::from("something else ");
    println!("{}", splice(s));
    println!("{}", splice(&s3));
}

fn splice(s2: &str) ->  &str {
    let bytes = s2.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s2[0..i];
        }
    }

    &s2[..]
}