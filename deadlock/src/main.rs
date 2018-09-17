use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let mx = Arc::new(Mutex::new(5));
    let my = Arc::new(Mutex::new(10));
    let mut handles = vec![];
    {
        let mx = Arc::clone(&mx);
        let my = Arc::clone(&my);
        handles.push(thread::spawn(move || {
            let _vx = mx.lock().unwrap();
            let _vy = my.lock().unwrap();
        }));
    }
    {
        let mx = Arc::clone(&mx);
        let my = Arc::clone(&my);
        handles.push(thread::spawn(move || {
            let _vy = my.lock().unwrap();
            let _vx = mx.lock().unwrap();
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
