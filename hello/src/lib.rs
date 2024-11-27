use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

struct Job;

pub struct ThreadPool {
    // 각 worker에서 handle 인스턴스를 갖게 됨
    workers: Vec<Worker>,      // `Worker` 스레드 풀
    sender: mpsc::Sender<Job>, // 송신자
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

        // 스레드간 통신 채널
        let (sender, receiver) = mpsc::channel();

        // receiver 공유 필요 (여러 `Worker`에서 공유)
        // -> 스마트 포인터 사용 (단, 여러 스레드에서 소유권을 공유하면서 값을 변경할 수 있어야 함)
        // -> `Arc<Mutex<T>>` : `Arc` 타입으로 여러 `Worker`에서 수신자 소유, `Mutex` 로 한 번에 한 워커에서만 사용 가능
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            // 스레드 실행을 먼저 한 뒤, 추후에 작업을 맡기고 싶음
            // `thread::spawn`: 스레드 생성 후 즉시 실행
            // 각 `Worker` 스레드에 receiver 등록
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        // 스레드 풀에 sender 등록
        ThreadPool { workers, sender }
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
    id: usize,                      // ID
    thread: thread::JoinHandle<()>, // 실제 스레드
}


impl Worker {
    // 구현 세부사항을 ThreadPool 등에게 알릴 필요가 없어, 비공개
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        // `Worker` 구조체는 `ThreadPool`에 보관된 대기열에서 실행할 코드를 가져와 자신의 스레드에서 실행하기를 원함
        // -> 스레드간 통신하는 방법을 사용
        let thread = thread::spawn(|| {
            // 클로저 내부에서 receiver 사용
            receiver;
        });


        Worker { id, thread }
    }
}
