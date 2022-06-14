
#[derive(Debug)]
enum UsState{   
    Alabama,
    Alaska,
    Ny,

}
enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin)-> u32{

    match coin{
        Coin:: Penny => 1,
        Coin:: Nickel => 5,
        Coin:: Dime => 10,
        Coin:: Quarter(state) => {

            println!("State Quater from {:?}!", state);
            25   
        },
    }
}

fn main(){

    let coin = Coin::Quarter(UsState::Ny);

    value_in_cents(coin);
}
