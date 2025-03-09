struct Rectangle {
    width: f64,
    height: f64
}

impl Rectangle {    
    fn area(&self) -> f64 {
        self.width * self.height
    }
    
    fn perimeter(&self) -> f64 {
        (self.width + self.height) * 2.0
    }
}

fn main() {
    let rect = Rectangle {
        width: 7.3,
        height: 3.8
    };

    // Calculate area of the rectangle
    println!("The area of a rectangle is: {} square units.", rect.area());

    // Calculate perimeter of the rectangle
    println!("The perimeter of a rectangle is: {} units.", rect.perimeter());
}
