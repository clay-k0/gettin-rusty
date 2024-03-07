use crossbeam_channel::unbounded;
use std::thread;

enum ThreadMsg {
    PrintData(String),
    Sum(i64, i64),
    Quit,
}

fn main() {
    let (s, r) = unbounded();
    let handle = thread::spawn(move || loop {
        match r.recv() {
            Ok(msg) => match msg {
                ThreadMsg::PrintData(d) => println!("{}", d),
                ThreadMsg::Sum(lhs, rhs) => println!("{} + {} = {}", lhs, rhs, (lhs + rhs)),
                ThreadMsg::Quit => {
                    println!("Quit | Thread Terminating...");
                    break;
                }
            },
            Err(e) => {
                println!("Error: {} | Disconnected", e);
                break;
            }
        }
    });

    s.send(ThreadMsg::PrintData("Hello from main!".to_owned()))
        .expect("Error sending message");
    s.send(ThreadMsg::Sum(153, 261))
        .expect("Error sending message");
    s.send(ThreadMsg::Quit).expect("Error sending message");

    handle.join().expect("Thread panicked");
}
