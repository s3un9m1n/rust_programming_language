//! 웹 서버 만들기
//!
use std::net::TcpListener;

fn main() {
    // 1. TCP 연결 수신 대기
    // - `bind`의 `Result<T, E>` 반환 중 에러 발생 시 `unwrap`에 의해 프로그램 종료
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // - `incoming`은 `TcpStream` 타입의 스트림 시퀀스를 제공하는 반복자 반환
    // - 하나의 스트림은 클라이언트와 서버간 개방형 연결(세션)을 나타냄
    for stream in listener.incoming() {
        // - 스트림에 에러가 있을 경우 `unwrap`에 의해 프로그램 종료 (ex. 동시 연결 수 제한)
        let stream = stream.unwrap();

        println!("Connection established!");
    }
}