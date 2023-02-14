use chrono::{DateTime, Local};

#[cfg(not(test))]
pub fn now() -> DateTime<Local> {
    chrono::Local::now()
}

#[cfg(test)]
pub use mock_time::now;

#[cfg(test)]
pub mod mock_time {
    use super::*;
    use std::cell::RefCell;

    thread_local! {
        static MOCK_TIME: RefCell<Option<DateTime<Local>>> = RefCell::new(None);
    }

    pub fn now() -> DateTime<Local> {
        MOCK_TIME.with(|cell| {
            cell.borrow()
                .as_ref()
                .cloned()
                .unwrap_or_else(chrono::Local::now)
        })
    }

    pub fn set_mock_time(time: DateTime<Local>) {
        MOCK_TIME.with(|cell| *cell.borrow_mut() = Some(time));
    }
}
