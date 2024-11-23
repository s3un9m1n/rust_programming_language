//! 웹 서버 만들기
//!
use std::{
    fs,
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
    // HTTP 첫 번째 요청 라인만 확인
    // 첫 번째 `unwrap`은 `Option`을 처리해 `next` 반복자의 아이템을 가져옴
    // 두 번째 `unwrap`은 `Result`를 처리해 기존 `map`의 `unwrap`과 동일한 효과
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    if request_line == "GET / HTTP/1.1" {
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("hello.html").unwrap();
        let length = contents.len();
    
        let response =
            format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    
        // `as_bytes()`: 문자열을 바이트로 변환
        // `write_all()`: `&u[8]`을 받아 연결(`stream`)쪽으로 직접 보냄
        stream.write_all(response.as_bytes()).unwrap();
    }
    else {
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = fs::read_to_string("404.html").unwrap();
        let length = contents.len();

        let response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
        );

        stream.write_all(response.as_bytes()).unwrap();
    }
}
