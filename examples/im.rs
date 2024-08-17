#![allow(unused)]

use std::{
    cell::Cell,
    sync::{Arc, Mutex},
};

#[derive(Debug)]
struct User {
    name: String,
    age: u32,
    active: Cell<bool>,
}

fn main() {
    /* let user = User {
        name: "Jordi".into(),
        age: 25,
        active: Cell::new(false),
    };

    println!("{user:?}");
    user.active.set(true);
    println!("{user:?}");

    // Mutex -> Mutual Exlusion

    let mutex = Mutex::new(1);

    println!("{mutex:?}");
    {
        let mut changer = mutex.lock().unwrap();
        println!("{mutex:?}");
        println!("{changer:?}");

        *changer += 1;
        println!("{changer:?}");
    }

    println!("{mutex:?}");
    {
        let mut other_changer = mutex.lock().unwrap();
        *other_changer += 1;
        println!("{mutex:?}");
    } */

    let n = Arc::new(Mutex::new(0));

    let n1 = Arc::clone(&n);
    let n2 = Arc::clone(&n);

    let thread1 = std::thread::spawn(move || {
        for _ in 0..10 {
            *n1.lock().unwrap() += 1;
            println!("t1")
        }
        // std::time::Duration::
        std::thread::sleep(std::time::Duration::from_secs(2))
    });

    let thread2 = std::thread::spawn(move || {
        for _ in 0..10 {
            *n2.lock().unwrap() += 1;
            println!("t2")
        }
        std::thread::sleep(std::time::Duration::from_secs(2))
    });

    thread1.join().unwrap();
    thread2.join().unwrap();
    println!("Value is {n:?}");
    println!("Exiting program")
}
