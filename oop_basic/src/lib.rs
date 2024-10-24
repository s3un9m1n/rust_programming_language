/// pub 구조체는 다른 코드가 이를 사용할 수 있도록 함
/// 내부 필드는 private으로 여전히 비공개임
/// 이를 통해 OOP의 캡슐화 수행
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    /// 평균을 외부에서 읽을 수는 있는 방법을 제공
    pub fn average(&self) -> f64 {
        self.average
    }

    /// list에 add / remove 될 때마다 자동으로 평균 갱신 후 캐싱\
    /// list만 직접 변경하거나 average만 직접 변경할 수 없음
    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}