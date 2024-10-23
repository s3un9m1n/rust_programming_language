use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Mutex<i32>는 i32가 아니므로 i32 값을 사용하기 위해서는 반드시 락을 얻어야 함
    let m = Mutex::new(5);

    {
        // 락을 얻으면 내부 데이터에 접근할 수 있음 (여기서는 num으로 받았음)
        // Mutex<T>는 스마트 포인터 (lock()의 반환값이 MutexGuard 라는 스마트 포인터 -> Deref가 구현되어 있어 내부 데이터 접근 가능)
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);

    // Mutex를 하나의 스레드로 전달하더라도 이후에 다른 스레드에서 접근하기 위함
    // Arc<T>(아토믹 참조 카운팅)는 동시적 상황에서 안전하게 사용할 수 있는 Rc<T> 같은 타입/
    // counter가 불변이지만 내부값의 가변 참조자를 가져올 수 있음
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // mutex를 이용해 여러 스레드에서 값을 공유
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
