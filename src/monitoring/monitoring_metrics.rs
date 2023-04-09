use lazy_static::lazy_static;
use prometheus::register_int_counter;
use prometheus::{self, IntCounter};

lazy_static! {
    pub static ref HIGH_FIVE_COUNTER: IntCounter =
        register_int_counter!("highfives", "Number of high fives received").unwrap();
}
