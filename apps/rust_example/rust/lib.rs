use std::sync::OnceLock;

static COUNT: OnceLock<BehaviorSubject<Double>> = OnceLock::new();

fn count_subject() -> Observable<Double> {
    COUNT.get_or_init(|| BehaviorSubject::new(0.0))
}

pub fn count() -> Observable<Double> {
    count_subject()
}

pub fn increment() {
    let count = count_subject();
    count.next(count.get() + 1.0);
}

pub fn format_count(prefix: String, value: Double) -> String {
    format!("{}: {}", prefix, value as i64)
}

pub fn label_bytes(label: String) -> Bytes {
    label.into_bytes()
}

pub fn payload_size(payload: Bytes) -> Double {
    payload.len() as Double
}
