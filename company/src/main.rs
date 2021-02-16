use std::collections::HashMap;

fn main() {
    loop_with_options();
}

fn loop_with_options() {
    // department -> Vector of Employees in that department
    let mut hm: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        println!("Choose an option!\n1> Add to company\n2> Print Department\n\
        3> Print Company\nAny other input will quit\n");

        let option = get_user_input();
        if option == "1" {
            println!("Enter someone into the company! Add person to department");
            let inputs = parse_user_input(get_user_input());
            add_to_company(&inputs.0, &inputs.1, &mut hm);
        }
        else if option == "2" {
            if hm.keys().len() <= 0 {
                println!("No departments founded!\nAdd people to the company!");
            }
            else {
                println!("Enter a department to print from the following...");
                for key in hm.keys() {
                    print!("{} ", key);
                }
                println!();
                let dept = String::from(get_user_input().trim());
                if hm.contains_key(&dept) {
                    print_department(&dept, &hm);
                }
                else {
                    println!("That department doesn't exist!");
                }
            }
        }
        else if option == "3" {
            print_company(&hm);
        }
        else {
            break;
        }
        println!();
    }
}

// grab user input from stdin
fn get_user_input() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    // unsafe, code input validation
    String::from(input.trim())
}

// break down the user input to get keywords
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
fn add_to_company(person: &str, department: &str, company: &mut HashMap<String, Vec<String>>) {
    let vector = company.entry(String::from(department)).or_insert(Vec::new());
    vector.push(String::from(person));
    vector.sort();
}

// print list of all people in a department
fn print_department(dept: &str, company: & HashMap<String, Vec<String>>) {
    // unsafe, check values
    let vector = company.get(dept).unwrap();
    println!("--------\nThe {} department:\n---------", dept);
    for employee in vector {
        println!("{}", employee);
    }
}

// print all people in company
fn print_company(company: & HashMap<String, Vec<String>>) {
    let mut list = Vec::new();
    for key in company.keys() {
        list.push(key);
    }
    list.sort();
    for dept in list {
        print_department(dept, company);
        println!("******************\n");
    }
}