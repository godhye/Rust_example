#[derive(Debug)]
struct Rect{

    len: u32,
    width: u32,
}

impl Rect{
    fn area(&self)->u32{
        self.len* self.width

    }

}
fn main() {

    let rect = Rect{
        len: 3,
        width: 4,

    } ;


    println!("area : {}" ,rect.area());
    println!("area : {:?}" ,rect);
    println!("{}" , "erer");


    
}
