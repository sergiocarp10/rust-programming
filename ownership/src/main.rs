fn main() {

    // create String from literal
    let s1 = String::from("hola mundo");
    
    // do a full copy (stack and heap content)
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    // tests
    test1();
    test2();
    test_borrow();
    test_borrow_2();
    dangle_test();
    test_slice();
}

// ========================= test 1 ================================= //

fn test1() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
    //println!("{}", s);            // ... and so is no longer valid here

    let mut x = 5;                  // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
    x = 20;                         // use x afterward
    println!("x = {}", x);

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

// ================================== test 2 ========================== //

fn test2() {
    let s1 = gives_ownership();         // gives_ownership moves its return
    println!("s1 = {}", s1);            // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
    println!("s3 = {}", s3);            // moves its return value into s3

} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}

// ================================== test 3 ============================= //

fn test_borrow() {
    let mut s = String::from("hello");

    // mutable references allows to mutate the value in borrowing context
    // restriction 1: only one mutable reference at a time
    // restriction 2: no mutable and immutable references at the same time

    // call a function and pass variable as mutable reference
    change(&mut s);

    // we still have ownership of s
    println!("{}", s);
}

fn change(some_string: &mut String) {
    // the function mutates the value it borrows
    some_string.push_str(", world");
}

// ================================ test 4 =============================== //

fn test_borrow_2(){

    // create mutable String
    let mut s = String::from("hola desde el test borrow 2");

    // create immutable references (no restriction)
    let imm1 = &s;
    let imm2 = &s;

    // use immutable references
    println!("imm1 = {}, imm2 = {}", imm1, imm2);

    // create mutable reference
    let m1 = &mut s;

    // use mutable reference (one at a time, no restriction)
    change(m1);
    println!("m1 = {}", m1);
}

// =============================== test 5 ================================= //

fn dangle_test(){
    let s2 = dangle();
    println!("s2 = {}", s2);
}

// returns a String variable
fn dangle() -> String {
    let s = String::from("test 5");

    // return value (no semicolon)
    // this transfers s' ownership to calling function
    s

    // if s' ownership is not moved, drop is called
    // &s       // references to nothing
}

// ================================= test 6 ================================ //

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn test_slice() {
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    println!("first word = {}", word);

    let word = first_word(&my_string[..]);      // whole string
    println!("first word = {}", word);

    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);

    // this causes a compilation error (we can't mutate an immutable variable)
    //my_string.clear();

    println!("first word = {}", word);

    // now with str (literal)

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    println!("first word = {}", word);

    let word = first_word(&my_string_literal[..]);
    println!("first word = {}", word);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
    println!("first word = {}", word);
}
