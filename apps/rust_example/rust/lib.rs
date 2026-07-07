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

pub fn describe_after_increment(formatter: DescribeAfterIncrementFormatterCallback) -> String {
    let count = count_subject();
    let next = count.get() + 1.0;
    count.next(next);
    formatter.call(next)
}

pub fn format_count(prefix: String, value: Double) -> String {
    format!("{}: {}", prefix, value as i64)
}

pub fn format_count_async(value: Double) -> Promise<String> {
    Promise::resolved(format!("Async Rust count: {}", value as i64))
}

pub fn label_bytes(label: String) -> Bytes {
    label.into_bytes()
}

pub fn payload_size(payload: Bytes) -> Double {
    payload.len() as Double
}

pub fn echo_payload(payload: CounterPayload) -> CounterPayload {
    let label = payload.get_label();
    let value = payload.get_value();
    CounterPayload::new(format!("{} echoed", label), value + 1.0)
}
