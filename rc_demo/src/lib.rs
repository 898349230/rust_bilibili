use std::cell::RefCell;

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T> 
where 
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T>{
        LimitTracker { messenger, value: 0, max, }
    }

    pub fn set_value(&mut self, value: usize){
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;
        if percentage_of_max >= 1.0 {
            self.messenger.send("percentage_of_max >= 1.0");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("percentage_of_max >= 0.9");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("percentage_of_max >= 0.75");
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockMessnger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessnger {
        fn new() -> MockMessnger {
            MockMessnger { sent_messages: RefCell::new(vec![]), }
        }
    }

    impl Messenger for MockMessnger {
        fn send(& self, msg: &str) {
            // borrow_mut() 获取内部值的可变引用
            self.sent_messages.borrow_mut().push(String::from(msg));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message () {
        let mock_messager = MockMessnger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messager, 100);

        limit_tracker.set_value(80);
        assert_eq!(mock_messager.sent_messages.borrow().len(), 1);
    }

}
