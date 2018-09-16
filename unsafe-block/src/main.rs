trait Messager {
    fn send(&self) {
        println!("Hello!");
    }
}

#[allow(dead_code)]
struct Window<'a> {
    notifier: Box<&'a Messager>,
    monitor: &'a str,
}

fn main() {
    struct MessageBox {};
    impl Messager for MessageBox {}

    let msg_box = MessageBox {};
    let monitor = "aoc";

    let window = Window {
        notifier: Box::new(&msg_box),
        monitor,
    };

    window.notifier.send();
}
