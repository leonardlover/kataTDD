use chrono::DateTime;
use std::cell::Cell;

thread_local! {
    static TIMESTAMP: Cell<i64> = const { Cell::new(0) };
}

pub fn set_timestamp(timestamp: i64) {
    TIMESTAMP.with(|ts| ts.set(timestamp));
}

pub struct Local;

impl Local {
    pub fn now() -> DateTime<chrono::Local> {
        DateTime::from_naive_utc_and_offset(
            TIMESTAMP.with(|timestamp| {
                DateTime::from_timestamp(timestamp.get(), 0)
                    .expect("invalid timestamp")
                    .naive_utc()
            }),
            chrono::Local::now().offset().clone()
        )
    }
}

