use std::sync::{LazyLock, Mutex};

static counter: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(0));

#[mu_sdk::function]
fn count() -> i32 {
    let mut c = counter.lock().unwrap();

    *c += 1;
    *c
}
