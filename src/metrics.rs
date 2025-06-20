use prometheus::{IntCounter, Histogram, register_int_counter, register_histogram};

lazy_static::lazy_static! {
    pub static ref HOGS_FETCHED_TOTAL: IntCounter =
        register_int_counter!("hogs_fetched_total", "Total number of hogs fetched").unwrap();
    pub static ref HOGS_SEARCHED_TOTAL: IntCounter =
        register_int_counter!("hogs_searched_total", "Total number of hogs searched").unwrap();
    pub static ref HOGS_CREATED_TOTAL: IntCounter =
        register_int_counter!("hogs_created_total", "Total number of hogs created").unwrap();
    pub static ref REQUEST_DURATION_SECONDS: Histogram =
        register_histogram!("request_duration_seconds", "Request duration in seconds").unwrap();
    
}
