enum  Shape {
    Rectangle(f64, f64),
    Circle(f64),
}

fn main() {
    let my_shape = Shape::Rectangle(1.0, 2.0);
    print_area(my_shape); 
    let circle = Shape::Circle(1.0);
    print_area(circle); 
     
} 

fn print_area(shape: Shape) -> f64 {
    let area = match  shape {
        Shape::Rectangle(a, b) => a * b,
        Shape::Circle(r) => 3.14 * r * r,
    };
    print!("{}", area);
    return area;

}