use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // mpsc: multiple producer, single consumer
    let (tx, rx) = mpsc::channel();

    // move를 통해 tx를 thread 내부로 소유권 이동
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // send() 시 val의 소유권이 넘어가기 때문에 밑에서는 사용 불가
        // println!("val is {}", val);

        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // rx는 main thread에서 소유
    // recv()와 달리 try_recv()는 MSG_PEEK과 같은 처리
    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    for received in rx {
        println!("Got: {}", received);
    }
}