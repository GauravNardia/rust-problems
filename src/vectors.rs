  

//   vector allows to store more than one value in single data structure that put all the values next to each other in memory.
// just like Array.
// {:?} – implementing debug trait 


fn main() {
    let mut vec = Vec::new();
     vec.push(1);
     vec.push(2);
     vec.push(3);
     vec.push(4);
     vec.push(5);
     vec.push(6);          

     println!("{:?}", even_filter(vec)) 
} 

fn even_filter(vec: Vec<i32>) -> Vec<i32> {
    let mut new_vec = Vec::new();
    for val in vec {
        if val % 2 == 0 {
            new_vec.push(val);
        }
    }
    return new_vec;
}