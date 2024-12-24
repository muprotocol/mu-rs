#[mu_sdk::public]
mod counter {
    use std::sync::{LazyLock, Mutex};

    static COUNTER: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(0));

    #[function]
    fn count() -> i32 {
        let mut c = COUNTER.lock().unwrap();

        *c += 1;
        *c
    }
}
