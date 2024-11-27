use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

/// `Job`
/// `execute` 메서드에서 수신하는 클로저 타입을 갖도록,
/// 트레이트 객체의 타입 별칭으로 변경
type Job = Box<dyn FnOnce() + Send + 'static>;

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
        // 클로저를 담는 `Box`(작업)를 채널로 보내기
        let job = Box::new(f);
        // `unwrap`: 전송 실패 시 종료
        // 사실, `ThreadPool`을 사용할 경우 모든 스레드의 실행이 중지되지는 않을 것이지만,
        // 컴파일러는 모르기 때문에 사용해야 함
        self.sender.send(job).unwrap();
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
        // `move`: 소유권 이동
        // `loop`: 작업 반복
        let thread = thread::spawn(move || loop {
            // 클로저 내부에서 receiver 사용
            // 참조만 하는 것이 아니라 실제 작업을 수신 후 처리 필요

            // `lock`: 뮤텍스 획득
            // 다른 스레드에서 `lock` 획득 후 패닉에 빠질 경우,
            // 뮤텍스가 독성(poisoned) 상태에 빠질 수 있어,
            // `unwrap`을 통해 패닉 유도
            // `recv`: 블록킹 방식으로 인해, 작업이 수신될 때까지 기다리게 될 것
            // 이때 `Mutex<T>` 덕분에 한 번에 하나의 `Worker`만 작업 요청 가능
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {id} got a job; executing.");

            job();
        });

        Worker { id, thread }
    }
}
