
// fn main () {
//     let ans;
//     let str1 = String::from("Small");
//     {
//         let str2 = String::from("longer");

//         ans = longest(str1, str2); 
//     }
//     println!("{}", ans); 
// }

// fn longest(a: String, b: String) -> String {
//     if a.len() > b.len() {
//         return a;
//     } else {
//         return b;
//     }
// }


// with Liftime generic anotation ('a)
// if 2 values lifetime are a and b then return values lifetime will be short values lifetime. intersection of both


fn main () {
    let ans;
    let str1 = String::from("Small");
    {
        let str2 = String::from("longer");

        ans = longest(&str1, &str2); 
    }
    println!("{}", ans); 
}

fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        return a;
    } else {
        return b;
    }
}