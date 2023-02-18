#[derive(Debug)]            // added to print state easily
enum UsState {
    Alabama,
    Alaska
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn main() {
    // example 1
    let my_coin = Coin::Quarter(UsState::Alaska);
    let cents_value = value_in_cents(my_coin);
    println!("Valor de la moneda: {} centavos", cents_value);

    // example 2
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("Value of six: {:?}", six);
    println!("Value of none: {:?}", none);

    // example 3
    let coin_2 = Some(Coin::Nickel);
    match_example(coin_2);
}

fn value_in_cents(coin: Coin) -> u8 {
    // match expression: no parenthesis needed
    // the arms' patterns must cover ALL possibilities
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1                               // return value, no semicolon
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("Quarter from {:?}", state);
            25
        }
    }
}

fn plus_one(num: Option<i32>) -> Option<i32> {
    // matches in Rust are EXHAUSTIVE
    match num {
        None => None,
        Some(i) => Some(i+1)
    }
}

fn match_example(coin: Option<Coin>) {

    match coin {
        Some(Coin::Penny) => println!("lucky penny!"),
        _ => println!("this is not a penny :/")     // otherwise, print this
    }

    // alternative using if let
    if let Some(Coin::Penny) = coin {
        println!("lucky penny from if let!");
    } else {
        println!("this is not a penny, from if let");
    }
}