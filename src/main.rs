struct Rectangle{
    width:u32,
    height:u32,
}


impl Rectangle{

    fn new(width:u32,height:u32)->Rectangle{
        Rectangle{
            width,
            height,
        }
    }


    fn area(&self)->u32{
        self.width*self.height
    }

    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
}


fn main() {
    let rect = Rectangle::new(30,50);

    println!("Area of rectangle is {}",rect.area());
}

