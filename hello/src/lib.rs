use std::thread;

pub struct ThreadPool {
    // 각 worker에서 handle 인스턴스를 갖게 됨
    workers: Vec<Worker>,
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

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            // 스레드 실행을 먼저 한 뒤, 추후에 작업을 맡기고 싶음
            // `thread::spawn`: 스레드 생성 후 즉시 실행
            workers.push(Worker::new(id));
        }

        ThreadPool { workers }
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

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    // 구현 세부사항을 ThreadPool 등에게 알릴 필요가 없어, 비공개
    fn new(id: usize) -> Worker {
        let thread = thread::spawn(|| {});

        Worker { id, thread }
    }
}