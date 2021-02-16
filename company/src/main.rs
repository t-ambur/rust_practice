use std::collections::HashMap;

fn main() {
    // department -> Vector of Employees in that department
    let mut hm: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        println!("Enter someone into the company! Add person to department");
        let inputs = parse_user_input(get_user_input());
        hm = add_to_company(&inputs.0, &inputs.1, hm);
        println!("The hashmap is this: {:?}", hm);
    }
}

fn get_user_input() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    // code input validation
    input
}

fn parse_user_input(input: String) -> (String, String) {
    let mut index = 0;
    let mut person = String::new();
    let mut department = String::new();
    for word in input.split_whitespace() {
        if index == 1 {
            person.push_str(word);
        }
        else if index == 3 {
            department.push_str(word);
        }
        index += 1;
    }
    (person, department)
}

// add employee (string) to vector
fn add_to_company(person: &str, department: &str, mut company: HashMap<String, Vec<String>>)
-> HashMap<String, Vec<String>> {
    let vector = company.entry(String::from(department)).or_insert(Vec::new());
    vector.push(String::from(person));
    company
} 

// code loop

// print list of all people in a department, alphabetically

// print all people in company, alphabetically