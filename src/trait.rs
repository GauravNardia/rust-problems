
trait Summary {
    fn sumarize(&self) -> String;
}

struct User {
    name: String,
    age: u32,
}

impl Summary for  User {
    fn sumarize(&self) -> String {
        return format!("The name is {} and age is {}", self.name, self.age); 
    }
    
}

fn main () {
    let user = User {
        name: String::from("Gaurav"),
        age: 21,
    };

    println!("{}", user.sumarize())
}

// use as a abstract class 