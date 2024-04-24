// threads3.rs
//
// Execute `rustlings hint threads3` or use the `hint` watch subcommand for a
// hint.


use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Queue {
    length: u32,
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            length: 10,
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

fn send_tx(q: Queue, tx: mpsc::Sender<u32>) -> () {
    let qc = Arc::new(q);
    let qc1 = Arc::clone(&qc);
    let qc2 = Arc::clone(&qc);
    let tx1 = tx.clone();
    let tx2 = tx.clone();

    thread::spawn(move || {
        for val in &qc1.first_half {
            println!("sending {:?}", val);
            tx1.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        for val in &qc2.second_half {
            println!("sending {:?}", val);
            tx2.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
}

fn main() {
    // let (tx, rx) = mpsc::channel();
    // let queue = Queue::new();
    // let queue_length = queue.length;

    // send_tx(queue, tx);

    // let mut total_received: u32 = 0;

    // // 创建两个新的通道，每个通道都有自己的接收者
    // let (tx1, rx1) = mpsc::channel::<T>();
    // let (tx2, rx2) = mpsc::channel();

    // // 启动两个线程，分别发送到不同的通道
    // let handle1 = thread::spawn(move || {
    //     for received in rx1 {
    //         total_received += 1;
    //         println!("Got: {}", received);
    //     }
    // });

    // let handle2 = thread::spawn(move || {
    //     for received in rx2 {
    //         total_received += 1;
    //         println!("Got: {}", received);
    //     }
    // });

    // // 主线程将发送者复制到两个线程中
    // let tx1_clone = tx.clone();
    // let tx2_clone = tx.clone();

    // // 发送数据到两个通道
    // thread::spawn(move || {
    //     for i in 1..=10 {
    //         if i <= 5 {
    //             tx1_clone.send(i).unwrap();
    //         } else {
    //             tx2_clone.send(i).unwrap();
    //         }
    //         thread::sleep(Duration::from_secs(1));
    //     }
    // });

    // // 等待两个线程完成
    // handle1.join().unwrap();
    // handle2.join().unwrap();

    // println!("total numbers received: {}", total_received);
    // assert_eq!(total_received, queue_length)
}
