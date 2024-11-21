//! 웹 서버 만들기
//!
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    // 1. TCP 연결 수신 대기
    // - `bind`의 `Result<T, E>` 반환 중 에러 발생 시 `unwrap`에 의해 프로그램 종료
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // - `incoming`은 `TcpStream` 타입의 스트림 시퀀스를 제공하는 반복자 반환
    // - 하나의 스트림은 클라이언트와 서버간 개방형 연결(세션)을 나타냄
    for stream in listener.incoming() {
        // - 스트림에 에러가 있을 경우 `unwrap`에 의해 프로그램 종료 (ex. 동시 연결 수 제한)
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines() // 줄바꿈 발견 때마다 데이터 스트림을 분할(`Result<String, std::io::Error>` 반복자 반환)
        .map(|result| result.unwrap()) // 각 `String` 뽑기 위해 `Result`를 `unwrap` (실제 프로덕션 코드에서는 적절한 에러 처리가 필요)
        .take_while(|line| !line.is_empty()) // HTTP 종료 지점 확인 (HTTP는 연속된 두 번의 줄바꿈으로 HTTP 요청의 끝을 알림)
        .collect(); // 라인들을 벡터에 수집

    // 수집된 라인을 디버그 형식을 사용해 출력
    println!("Request: {:#?}", http_request);
}
