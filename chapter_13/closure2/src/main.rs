

fn main() {
    fn  add_one_v1   (x: u32) -> u32 { x + 1 }
    let add_one_v4 = |x|x + 1  ;

    println!("add_one_v1: {}", add_one_v1(5));
    println!("add_one_v4: {}", add_one_v4(5));
}

