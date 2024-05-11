use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            println!("Send msg: {}", &val);
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });
    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];
        for val in vals {
            println!("Send msg: {}", &val);
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    println!("Receiving...");
    for received in &rx {
        println!("Got: {}", received);
    }

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter_cloned = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter_cloned.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
