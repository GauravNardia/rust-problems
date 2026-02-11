struct Rec {
    height: i32,
    width: i32,
}

impl Rec {
    fn area(&self) -> i32 {
        return self.width * self.height;
    }
    fn parameter(&self) -> i32 {
       return  2 * (self.width + self.height)
    }
}

fn main() {
    let  rect = Rec {
        height: 3,
        width: 4
    };

    print!("{}", rect.area());
    println!("{}", rect.parameter());

}