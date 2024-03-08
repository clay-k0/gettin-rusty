#![allow(warnings)]
use crossbeam_channel::unbounded;
use std::thread;

enum WorkerMsg {
    PrintData(String),
    Sum(i64, i64),
    Quit,
}

enum MainMsg {
    SumResult(i64),
    WorkerQuit,
}

fn main() {
    let (worker_tx, worker_rx) = unbounded();
    let (main_tx, main_rx) = unbounded();

    let worker = thread::spawn(move || loop {
        match worker_rx.recv() {
            Ok(msg) => match msg {
                WorkerMsg::PrintData(d) => println!("{}", d),
                WorkerMsg::Sum(lhs, rhs) => {
                    println!("Worker | Summing...");
                    main_tx.send(MainMsg::SumResult(lhs + rhs));
                    ()
                }
                WorkerMsg::Quit => {
                    println!("Worker | Thread Terminating...");
                    main_tx.send(MainMsg::WorkerQuit);
                    break;
                }
            },
            Err(e) => {
                println!("Worker | Disconnected Due to Error");
                main_tx.try_send(MainMsg::WorkerQuit);
                break;
            }
        }
    });

    worker_tx
        .send(WorkerMsg::PrintData("Hello from main!".to_owned()))
        .expect("Thread panicked");
    worker_tx
        .send(WorkerMsg::Sum(316, 84))
        .expect("Thread panicked");
    worker_tx.send(WorkerMsg::Quit).expect("Thread panicked");

    while let Ok(msg) = main_rx.recv() {
        match msg {
            MainMsg::SumResult(sum) => println!("Main | Sum was: {}", sum),
            MainMsg::WorkerQuit => println!("Main | Worker Terminated"),
        }
    }

    worker.join().expect("Thread panicked");
}
