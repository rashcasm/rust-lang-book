mod front;

pub use crate::front::meow;

fn main() {
    println!("Hello, world!");
    meow::something();
}
