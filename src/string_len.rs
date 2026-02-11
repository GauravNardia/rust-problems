fn main() {
    let name = String::from("Gaurav");
    let length = get_str_len(name);
    println!("the length of the string is {}", length);
}
 
fn get_str_len(str: String) -> usize {
    return str.chars().count();
}