#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    let rect2 = (10, 40);

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!(
        "The area of the rectangle using raw variables is {} square pixels.",
        area1(width1, height1)
    );

    println!(
        "The area of the rectangle using a tuple is {} square pixels.",
        area2(rect2)
    );

    println!(
        "The area of the rectangle using struct Rectangle is {} square pixels.",
        area3(&rect3)
    );

    println!("rect3 is: {:?}", rect3);

    println!(
        "The area of the rectangle using Rectangle.area() is {} square pixels.",
        rect3.area()
    );

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

fn area1(width: u32, height:u32) -> u32 {
    // using raw width and height, less useful cause not intuitive that they're related
    width * height
}

fn area2(dimensions: (u32, u32)) -> u32 {
    // Now they're related, but we don't know what we're taking in dimensions of
    dimensions.0 * dimensions.1
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}