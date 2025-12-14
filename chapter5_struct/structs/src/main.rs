#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
fn main() {
    let meowmeow = Rectangle {
        width: 30,
        height: 50,
    };
//    print!("{:#?}", meowmeow);
    dbg!(&meowmeow);
    println!(
        "\nThe area of the rectangleis {} square pixels.", meowmeow.area()
    );
}