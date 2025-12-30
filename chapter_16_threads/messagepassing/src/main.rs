use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).expect("something");
    });

    let received = rx.recv().unwrap();
    println!("Got: {received}");
}

// “Do not communicate by sharing memory;
// instead, share memory by communicating.”