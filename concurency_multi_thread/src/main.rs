use std::thread;
use std::time::Duration;

fn main() {
    // 스레드 생성해 작업 주기
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // 스레드 종료 대기
    handle.join().unwrap();

    let v = vec![1, 2, 3];

    // 클로저 앞에 move를 추가해 값을 추론해 빌려오는 것이 아니라 소유권을 강제로 갖도록 함
    let handle = thread::spawn(move || {
        // 클로저에 인수가 없음 -> 메인 스레드로부터 어떤 데이터도 이용하지 않음
        // 그렇지만 v를 사용하고 있기에 캡처되어 클로저 환경의 일부가 됨
        // 그러나 v를 빌릴 때 생성된 스레드가 얼마나 오랫동안 실행될 지 알 수 없기 때문에, v에 대한 참조작 ㅏ항ㅅ아 유효한지 알 수 없다
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
