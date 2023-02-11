fn main() {

    // create String from literal
    let s1 = String::from("hola mundo");
    
    // do a full copy (stack and heap content)
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}
