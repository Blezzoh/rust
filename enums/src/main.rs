enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(String), // state
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("state for quarter is {:?}", state);
            25
        },
    }
}
// alternative using if let
fn alternative_values_in_cents(coin: &Coin) -> u8{
    // let mut value = 0;
    if let Coin::Penny = coin{
        // value = 1;
        1
    }
    else if let Coin::Nickel = coin{
        // value = 5;
        5
    }
    else if let Coin::Quarter(state) = coin{
        println!("state for quarter is {:?}", state);
        // value = 25;
        25
    }
    else{
        // value = 10;
        10
    }
    // value
}

fn main() {
    
    let coin = Coin::Quarter(String::from("Oregon"));
    let coin2 = Coin::Penny;
    let coin3 = Coin::Dime;
    let coin4 = Coin::Nickel;
    println!("Coin values in cents using match are {}, {}, {}, {}", value_in_cents(&coin), value_in_cents(&coin4), value_in_cents(&coin3), value_in_cents(&coin2));
    println!("Coin values in cents using if let are {}, {}, {}, {}", alternative_values_in_cents(&coin), alternative_values_in_cents(&coin4), alternative_values_in_cents(&coin3), alternative_values_in_cents(&coin2));
}
