use io_enhance_iterator::Config;
use std::{env, process};

// 예제 12.6 기준 개선
fn main() {
    // let args: Vec<String> = env::args().collect();
    // let config = Config::build(&args).unwrap_or_else(|err| {

    // 반복자를 직접 사용
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // --생략--
}
