use std::collections::HashMap;

fn main() {
    let a = [1,2,3];
    let mut v:Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    let v2 = vec![1,2,3];
    dbg!(a);
    dbg!(v);
    dbg!(&v2);

    match &v2.get(20){
        Some(x) => println!("The 20th element is {}", x),
        None => (),
    }

    let blue = String::from("Blue");
    let yellow = String::from("Yellow");
    let mut scores = HashMap::new();
    scores.insert(blue, 10);
    scores.insert(yellow, 50);
    dbg!(&scores);
}
// rest in rustlings
