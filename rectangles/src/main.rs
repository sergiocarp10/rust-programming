#[derive(Debug)]        // needed to use {:?} on println! macro
struct Rectangle {
    width: u32,
    height: u32
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

    // show dimensions in one line
    println!("rect1 is {:?}", rect1);

    // show each attribute by line
    println!("rect1 is {:#?}", rect1);

    // calculate area
    println!(
        "The area of the rectangle is {} square pixels.",
        area_v3(&rect1)
    );
}

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