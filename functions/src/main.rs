fn main() {
    print_labeled_measument(5, 'L');

    // Expression example
    let y = {
        let z = 2 + 3;
        z + 5           // expression return (no semicolon)
    };

    println!("The value of y is: {y}");

    // Function with return value example
    let x = five();
    println!("The value of x is: {x}");
}

// declaring the type of each parameter is mandatory
fn print_labeled_measument(value: i32, unit_label: char){
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5                   // "return" word isn't needed
}