#[derive(Debug)]        // needed to use {:?} on println! macro
struct Rectangle {
    width: u32,
    height: u32
}

// struct methods
impl Rectangle {
    fn valid(&self) -> bool {
        self.width > 0 && self.height > 0
    }

    fn can_hold(&self, another: &Rectangle) -> bool {
        self.width > another.width && self.height > another.height
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }
}

// struct constructors
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }
}


fn main() {
    // let width1 = 40;
    //let height1 = 50;
    // let rect = (40, 50);
    
    // struct instance is created using brackets
    let rect1 = Rectangle {
        width: 40, 
        height: 50 
    };

    // using constructor (by ::)
    let rect2 = Rectangle::square(35);

    // show dimensions in one line
    println!("rect1 is {:?}", rect1);

    // show each attribute by line
    println!("rect1 is {:#?}", rect1);

    // check dimensions
    if rect1.valid() && rect2.valid() {
        println!("both rectangles are valid");
    }

    // calculate area
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    // compare rectangles
    println!("can rect1 hold rect2? {}", rect1.can_hold(&rect2));
}

/*
fn area_v1(width: u32, height: u32) -> u32 {
    width * height
}

fn area_v2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// immutable borrow: we won't take ownership of the struct
fn area_v3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
*/