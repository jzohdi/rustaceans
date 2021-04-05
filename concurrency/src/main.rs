use std::thread;
use std::time::Duration;
use std::sync::mpsc; // multiple producer single consumer
use std::sync::{Arc, Mutex};

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let final_count = counter.lock().unwrap();
    println!("Result: {}", *final_count);
}

fn message_recv_2() {
    let (tx, rx) = mpsc::channel();

    let tx2 = tx.clone(); // .clone will make second transmitter for the channel
    thread::spawn(move || {
        for val in 0..5 {
            tx.send(String::from("1")).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        for val in 0..5 {
            tx2.send(String::from("2")).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for recv in rx {
        println!("Message receieved from: {}", recv);
    }
}

fn receive_iter() {

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hello"),
            String::from("from"),
            String::from("the"),
            String::from("spawned"),
            String::from("thread")];
        
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("got: {}", received);
    }
}

// this function would fail to compile because
// ownership is transfered when the message is sent. 
// The "Sent message {}" println violates the ownership rules 
fn thread_message_ownership() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hello world");
        tx.send(val).unwrap(); // send returns Option which is Ok is the channel is still open
//        println!("Sent message: {}", val);
    });

    let message = rx.recv().unwrap();
    println!("Got message: {}", message);
}

fn spawn_move() {
    let v = vec![1, 2, 3];

    // need to use move so that the thread takes ownership of v
    // otherwise rust compiler would not be able to tell
    // if the reference to v (println!) would be valid for the 
    let handle = thread::spawn(move || {
        println!("v = {:?}", v);
    });

    handle.join().unwrap();

}

fn basics_1() {
    
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hellow number {} from main thread", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
