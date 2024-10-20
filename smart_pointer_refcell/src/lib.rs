//! 테스트 시나리오
//! 최대값을 기준으로 어떤 값을 추적하여 현재 값이 최대값에 얼마나 근접했는지 메시지를 전송하는 라이브러리
//! 라이브러리는 한 명의 사용자에게 허용되고 있는 API 호출 수의 허용량을 추적하는데 사용 가능
//! 이 라이브러리를 사용하는 애플리케이션에서 메시지를 전송하는 것에 대한 메커니즘은 제공이 필요 (이메일, 문자 등)

/// 목 객체와 실제 객체 모두 Messenger 트레이트를 구현해야 함
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        // sent_messages: Vec<String>,
        // sent_messages를 RefCell<T> 내에 저장하면 send 메서드에서는 sent_messages를 수정할 수 있음
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                // sent_messages: vec![],
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // send가 self의 불변참조자를 가져오기 때문에 sent_messages를 변경할 수 없음
            // 시그니처의 인자로 지정되어 있기 때문에 &mut self로도 변경할 수 없음
            // self.sent_messages.push(String::from(message));
            // RefCell<T>의 가변 참조자 대여 (RefMut<T> 반환: Deref 구현되어있음)
            // 가변 참조자는 같은 스코프에서 단 하나만 존재할 수 있음 (어길 시 panic! 발생)
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        // assert_eq!(mock_messenger.sent_messages.len(), 1);
        // RefCell<T>의 불변 참조자 대여 (Ref<T> 반환: Deref 구현되어있음)
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}