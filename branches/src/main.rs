fn main() {
    simple_handling_test();
    multiple_handling_test();
    if_let_test();
    //infinite_loop();
    //return_value_from_loop();
    loop_label_test();
    looping_through_collection_with_while();
    looping_through_collection_with_for();
}

fn simple_handling_test(){
    let x = 20;

    // Rust doesn't convert numbers to boolean unlike Ruby or JS
    if x < 10 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if x != 15 {
        println!("x was something other than 15");
    }
}

fn multiple_handling_test(){
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn if_let_test(){
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}

fn infinite_loop(){
    loop {
        println!("again!");
    }
}

fn return_value_from_loop(){
    let mut counter = 0;

    // the return value is after the break expression
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn loop_label_test() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                // this statement will exit the inner loop only
                break;
            }
            if count == 2 {
                // this statement will exit the outer loop
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn looping_through_collection_with_while() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}

fn looping_through_collection_with_for() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}
