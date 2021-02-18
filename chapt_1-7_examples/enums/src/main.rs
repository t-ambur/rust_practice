fn main() {
    let mut five = Some(5);
    if let Some(5) = five {
        five = Some(0);
    }
    println!("{:?}", five);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}