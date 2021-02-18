use std::collections::HashMap;

fn main() {
    // sorted    1  2  2  3  4  4  5
    let v = vec![1, 2, 3, 4, 5, 4, 2, 4, 6, 9, 11];
    println!("The mode is {:?}", mode(&v));
    println!("The mean is {:?}", mean(&v));
    println!("The median is {:?}", median(&v));
}

// my interp of mode is to return the most frequent value if it appears more
// than one time.
fn mode(vector: &Vec<i32>) -> Vec<i32> {
    let mut hm = HashMap::new();

    let mut highest_value: i32 = 0;
    let mut highest_keys: Vec<i32> = Vec::new();

    for num in vector {
        let count = hm.entry(num).or_insert(0);
        *count += 1;
        if *count > highest_value {
            highest_value = *count;
        }
    }
    if highest_value <= 1 {
        return highest_keys
    }
    for (key, value) in hm {
        if value == highest_value {
            if !highest_keys.contains(key) {
                highest_keys.push(*key);
            }
        }        
    }
    highest_keys
}

fn mean(vector: &Vec<i32>) -> f32 {
    let mut sum = 0;
    for number in vector {
        sum += *number;
    }
    sum as f32 / vector.len() as f32
}

fn median(vector_in: &Vec<i32>) -> f32 {
    let mut vector = vector_in.clone();
    vector.sort();
    // if divisible by 2, then even
    if vector.len() % 2 == 0 {
        let end_index = vector.len() / 2;
        return (vector[end_index-1] as f32 +vector[end_index] as f32) / 2.0;
    }
    // else odd, return truncated half
    vector[(vector.len() / 2)] as f32
}