#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    fn max(self, other: Rectangle) -> Rectangle {
        Rectangle {
            width: self.width.max(other.width),
            height: self.height.max(other.height)
        }
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50
    };

    // Print instance using debug formatting to stdout
    println!("rect1 is {rect1:#?}");

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 50
    };

    // Print instance to stderr
    dbg!(&rect2);

    let mut rect3 = Rectangle::square(40);
    
    rect3.set_width(20);

    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    rect3 = rect3.max(rect2);
}