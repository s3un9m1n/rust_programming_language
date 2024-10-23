use std::sync::{Arc, Mutex};
use std::thread;

//! 동시성 제어를 위해서는 Send / Sync 마커 트레이트에 대해서 알아야 함
//! Send는 스레드간 소유권을 이전할 수 있냐를 마크 (move)
//! Sync는 스레드간 값을 공유할 때 동시에 참조할 수 있느냐를 마크
//! 즉, 소유권 이전과 공유에서 차이가 있고, 동시에 구현이 돼있을 수 있음
//!
//! 스레드간 데이터를 넘길 때 구조체 자체를 넘길 수도 있음
//! 하지만, 구조체 내부 값이 모두 Send 트레이트를 마크하고 있어야 함

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
