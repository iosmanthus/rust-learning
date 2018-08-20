use std::sync::mpsc;
use std::thread;
use std::time::Duration;
fn main() {
    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);
    //let tx1 = tx.clone();

    thread::spawn(move || {
        let msgs = vec![
            String::from("a"),
            String::from("b"),
            String::from("c"),
            String::from("d"),
            String::from("e"),
        ];
        for m in msgs {
            tx1.send(m).unwrap();
        }
    });

    thread::spawn(move || {
        let msgs = vec![
            String::from("f"),
            String::from("g"),
            String::from("h"),
            String::from("i"),
            String::from("j"),
        ];
        for m in msgs {
            tx.send(m).unwrap();
        }
    });

    for m in rx.iter() {
        println!("{}", m);
    }
}
