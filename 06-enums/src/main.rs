enum IpAddrKind{
    V4(u8,u8,u8,u8),
    V6(String),
}

enum Message{
    Quit,
    Move,
    Write(String),
    ChangeColor(i32),
}
 impl Message {
     fn print_msg(){
        println!("Hello from message impl");
     }
 }

//  enum Option<T>{
//     Some(T),
//     None,
//  }



fn main() {
    let localhost = IpAddrKind::V4(127,0,0,1);
    let six = IpAddrKind::V6(String::from("2001:0db8:0000:0000:0000:8a2e:0370:7334 becomes 2001:db8::8a2e:370:7334")); 

    


}


enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
        _=> {                       // Default case
            println!("Not a coin");
            0
        },
    }
}
