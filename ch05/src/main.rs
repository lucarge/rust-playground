struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, rectangle: &Rectangle) -> bool {
        self.height > rectangle.height && self.width > rectangle.width
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            height: size,
            width: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        height: 50,
        width: 30,
    };
    let rect2 = Rectangle {
        height: 40,
        width: 10,
    };
    let rect3 = Rectangle::square(80);

    println!("The area of the rectangle is {} square pixels.", rect1.area());
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
