use std::env;

pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    // pub fn build(args: &[String]) -> Result<Config, &'static str> {
    // build 시그니처 변경 (반복자를 받도록)
    // 트레이트 바운드(Iterator<Item>)를 사용한 제네릭 타입으로, String 아이템을 반환하는 타입으로 지정됨
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // 반복자를 사용함으로써 인자의 개수를 세는 등의 동작이 없고 보다 명확하게 다룰 수 있음
        args.next();

        // 반복자를 이용해 소유권 이전을 하며, 이전과 같이 clone() 과정이 없음
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}
