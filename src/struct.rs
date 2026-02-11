 struct User {
    first_name: String,
    last_name: String,
    age: u32,
 }

 fn main() {
    let user = User {
        first_name: String::from("Gaurav"),
        last_name: String::from("Nardia"),
        age: 21
    };

    print!("{}", user.first_name);
 }