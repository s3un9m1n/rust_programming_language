use std::thread;

pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut threads = Vec::with_capacity(size);

        for _ in 0..size {
            // create some threads and store them in the vector
        }

        ThreadPool { threads }
    }

    // 한 번만 호출될 것이기 때문에 `FnOnce` 가 사용되고자 하는 트레이트
    pub fn execute<F>(&self, f: F)
    // FnOnce() 트레이트 바운드: thread::spawn 에 전달하기 위함 (매개변수 없이 유닛타입 반환)
    // Send 트레이트 바운드: 한 스레드에서 다른 스레드로 클로저를 전송하기 위함
    // 'static 라이프타임 바운드: 스레드가 실행되는 데 걸리는 시간을 모르기 때문에 필요
    where
        F: FnOnce() + Send + 'static,
    {

    }
}