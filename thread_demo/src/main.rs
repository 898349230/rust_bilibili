use std::thread;
use std::time::Duration;
use std::sync::{mpsc, Mutex};

fn main() {

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!(" thread spawn num is {}", i);
            // thread::sleep(Duration::from_millis(1000));
        }
    });

    for i in 1..5 {
        println!(" thread main num is {}", i);
        // thread::sleep(Duration::from_millis(1000));
    }

    handle.join().unwrap();


    let v = vec![1,2,3];

    // 使用 move 关键字将 v 的所有权转移
    let handler2 = thread::spawn(move || {
        println!("here is a vector : {:?}", v);
    });

    handler2.join().unwrap();

    // 线程间通信
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let str = String::from("hello");
        tx.send(str).unwrap();
    });
    //  会一直阻塞，直到 channel 发送过来消息， 返回 Result<T,E>
    let reveived = rx.recv().unwrap();
    println!("reveived : {}", reveived);

    let (tx2, rx2) = mpsc::channel();
    // 克隆多个发送者
    let tx2Clone = mpsc::Sender::clone(&tx2);
    thread::spawn(move || {
        let msgs = vec![
            String::from("1. a"),
            String::from("1. b"),
            String::from("1. c"),
        ];
        for msg in msgs {
            tx2.send(msg).unwrap();
            thread::sleep(Duration::from_millis(300));
        }
    });    
    thread::spawn(move || {
        let msgs = vec![
            String::from("2. a"),
            String::from("2. b"),
            String::from("2. c"),
        ];
        for msg in msgs {
            tx2Clone.send(msg).unwrap();
            thread::sleep(Duration::from_millis(300));
        }
    });

    for msg in rx2 {
        println!("reveived : {}", msg);
    }


    // mutex
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    } // 到这里自动解锁
    println!("m = {:?}", m);

}
