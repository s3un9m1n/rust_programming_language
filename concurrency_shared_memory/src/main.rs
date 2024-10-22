use std::sync::Mutex;

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
}