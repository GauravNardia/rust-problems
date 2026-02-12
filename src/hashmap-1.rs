use std::collections::HashMap;

  

fn main() {
    let mut users: HashMap<String, u32> = HashMap::new();
    users.insert(String::from("Gaurav"), 21);
    users.insert(String::from("Elon"), 56);

    let first_user_age = users.get("musk");

    match first_user_age {
        Some(age) => println!("User's age {:?}", age),
        None => println!("User not found"),
    }
    
} 
