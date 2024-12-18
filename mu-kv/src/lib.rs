use std::{
    collections::HashMap,
    sync::{LazyLock, Mutex},
};

use ic_principal::Principal;

static COLLECTIONS: LazyLock<Mutex<HashMap<(Principal, String), Collection>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

type Collection = HashMap<String, String>;

#[ic_cdk::query]
fn list_collections() -> Vec<String> {
    let collections = COLLECTIONS.lock().unwrap();
    let caller = ic_cdk::caller();
    collections
        .iter()
        .filter_map(|((principal, name), _)| {
            if principal == &caller {
                Some(name.clone())
            } else {
                None
            }
        })
        .collect()
}

#[ic_cdk::query]
fn has_collection(name: String) -> bool {
    let collections = COLLECTIONS.lock().unwrap();
    let caller = ic_cdk::caller();
    collections.contains_key(&(caller, name))
}

#[ic_cdk::update]
fn create_collection(name: String) {
    let caller = ic_cdk::caller();
    let mut collections = COLLECTIONS.lock().unwrap();
    collections.insert((caller, name), HashMap::new());
}

#[ic_cdk::query]
fn query_collection(name: String, key: String) -> Option<String> {
    let collections = COLLECTIONS.lock().unwrap();
    let caller = ic_cdk::caller();
    collections
        .get(&(caller, name))
        .and_then(|collection| collection.get(&key).cloned())
}

#[ic_cdk::query]
fn query_all_keys(name: String) -> Vec<String> {
    let collections = COLLECTIONS.lock().unwrap();
    let caller = ic_cdk::caller();
    collections
        .get(&(caller, name))
        .map(|collection| collection.keys().cloned().collect())
        .unwrap_or_default()
}

#[ic_cdk::query]
fn query_all_values(name: String) -> Vec<String> {
    let collections = COLLECTIONS.lock().unwrap();
    let caller = ic_cdk::caller();
    collections
        .get(&(caller, name))
        .map(|collection| collection.values().cloned().collect())
        .unwrap_or_default()
}

#[ic_cdk::update]
fn update_collection(name: String, key: String, value: Option<String>) {
    let mut collections = COLLECTIONS.lock().unwrap();
    let caller = ic_cdk::caller();
    if let Some(collection) = collections.get_mut(&(caller, name)) {
        if let Some(value) = value {
            collection.insert(key, value);
        } else {
            collection.remove(&key);
        }
    }
}

#[ic_cdk::update]
fn delete_collection(name: String) {
    let mut collections = COLLECTIONS.lock().unwrap();
    let caller = ic_cdk::caller();
    collections.remove(&(caller, name));
}

ic_cdk::export_candid!();
