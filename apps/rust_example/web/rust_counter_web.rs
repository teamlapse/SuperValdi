#[unsafe(no_mangle)]
pub extern "C" fn rust_counter_increment(value: f64) -> f64 {
    value + 1.0
}

fn main() {}
