
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

fn value_in_cents(coin: &Coin)-> u32{
    
    match coin{
        Coin:: Penny => 1,
        Coin:: Nickel => 5,
        Coin:: Dime => 10,
        Coin:: Quarter(state) => {
            25   
        },
    } 
}

fn plus_1(x: Option<i32>) -> Option<i32>{

    match x {
        None => None,
        Some(i) => Some(i+1),

    }
}

fn main(){

    let coin = Coin::Quarter(UsState::Ny);

    
    
    if let Coin::Quarter(state) = coin {
        println!("State Quater from {:?}!", state);
    } 

    let total = value_in_cents(&coin);


    let some_num = Some(5);
    let plusone = plus_1(some_num);

    if let Some(6) = plusone{
        println!("6");
    }
    let absent_num:Option<i32> = None;



}