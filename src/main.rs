struct Circle{
    radius:f32, 
}


impl Circle{

    fn new(radius:f32)->Circle{
        Circle{
            radius,
        }
    }


    fn area(&self)->f32{
        self.radius * self.radius * 3.14159
    }

    fn perimeter(&self) -> f32 {
        2.0 * self.radius * 3.14159
    }
}


fn main() {
    let rect = Circle::new(30.0);

    println!("Area of Circle is {}",rect.area());
    println!("Perimeter of Circle is {}",rect.perimeter());
}

