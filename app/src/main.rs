use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

fn main() {
    let mut a = 2;
    let s = "Hello World";
    let s1 = String::from("Hello World");

    a += 1; // Needs mut
    print_stacked_int(a);
    print_stacked(s);
    print_heaped(&s1); // ownership

    let rc_a = Rc::new(1);
    println!("{}", Rc::strong_count(&rc_a));
    let rc_b = rc_a.clone();
    println!("{}", Rc::strong_count(&rc_a));

    print_stacked_int(*rc_a);
    print_stacked_int(*rc_b);

    let arc_a = Arc::new(1); // Thread-safe reference count
    for _ in 1..5 {
        let a_cloned = arc_a.clone();
        thread::spawn(move || {
            println!("Hello from thread {:?}. Number {}", thread::current().id(), a_cloned);
        }).join().unwrap();
    }

    // atomic variant
    let arc_b = Arc::new(Mutex::new(1));
    let mut threads: Vec<JoinHandle<_>> = Vec::new();

    for _ in 1..5 {
        let b_cloned = arc_b.clone();
        let t = thread::spawn(move || {
            let id = thread::current().id();
            println!("Thread {:?}: Started.", id);
            let mut loops = 1;
            loop {
                match b_cloned.lock() {
                    Ok(mut v) => {
                        *v += 1;
                        if *v > 25 { break; }
                    }
                    Err(e) => { println!("Error {}", e); }
                }
                println!("Thread {:?}: Number {:?}, Loop {}", id, b_cloned, loops);
                thread::sleep(Duration::from_millis(500));
                loops += 1;
                if loops > 5 {
                    println!("Thread {:?}: Break", id);
                    break;
                }
            }
        });
        threads.push(t);
    }

    for t in threads {
        let id = t.thread().id();
        println!("Waiting for Thread {:?} to stop...", id);
        t.join().unwrap();
        println!("Thread {:?} Stopped.", id);
    }
}

fn print_stacked_int(s: i32) {
    println!("{}", s);
}

fn print_stacked(s: &str) {
    println!("{}", s);
}

fn print_heaped(s: &String) {
    println!("{}", s);
}
