enum Shape{
    Rectangle(f64, f64), 
    Circle(f64)
}

fn main(){
    let my_shape = Shape::Rectangle(1.3,2.1);
    let rect_area = calculate_area(my_shape);
    println!("Area of rectangle {}", rect_area);
    let my_shap = Shape::Circle(4.3);
    let circle_area = calculate_area(my_shap);
    println!("Area of Circle {}", circle_area);
}

fn calculate_area(shape:Shape) -> f64{
    match shape {
        Shape::Rectangle(a,b) => a*b,
        Shape::Circle(a) => 3.14*a*a,
    }
}