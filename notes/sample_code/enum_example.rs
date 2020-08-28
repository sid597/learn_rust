enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter
}

fn value_in_cents(coin:Coin) -> i8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quarter => 15
    }
}


fn main () {
    enum IpAddr{
        V4(String),
        V6(String)
    } 

    let home = IpAddr::V4(String::from("137.0.0.1"));
    let work = IpAddr::V6(String::from("123.456.789.123.456.789"));
   
    let c = value_in_cents(Coin::Dime);
    println!("Value in cents for Dime is {}", c);

}