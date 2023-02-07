fn main() {
    // test_shadowing();
    
    test_tuples();
}

// don't forget: shadow_case recommended
fn test_shadowing(){
    let x = 5;

    // this is shadowing: we can change the type but reuse the same name
    // mut doesn't allow to change the type
    let x = x + 1;

    {
        let x = x * 2;

        // this prints 12
        println!("The value of x in the inner scope is: {x}");
    }

    // this prints 6
    println!("The value of x is: {x}");
}

fn test_tuples(){
    let tup = (500, 6.4, 1);
    
    // destructure a tuple value
    let (x, y, z) = tup;
    println!("The values are: {x}, {y} and {z}");
}