use ic_principal::Principal;
use std::{cell::LazyCell, future::Future};

use crate::MuError;

pub trait MuKv {
    fn collection(&self, name: &str) -> impl Future<Output = Result<impl MuKvCollection, MuError>>;
}

pub trait MuKvCollection {
    fn query(&self, key: &str) -> impl Future<Output = Result<Option<String>, MuError>>;
    fn update(&self, key: &str, value: Option<&str>) -> impl Future<Output = Result<(), MuError>>;
    fn keys(&self) -> impl Future<Output = Result<Vec<String>, MuError>>;
    fn values(&self) -> impl Future<Output = Result<Vec<String>, MuError>>;
    fn delete(&self) -> impl Future<Output = Result<(), MuError>>;
}

pub struct MuIcpKv;

const KV_PRINCIPAL: LazyCell<Principal> =
    LazyCell::new(|| "bkyz2-fmaaa-aaaaa-qaaaq-cai".parse().unwrap());

impl MuIcpKv {
    pub fn new() -> Self {
        Self
    }
}

struct MuIcpKvCollection {
    name: String,
}
impl MuKv for MuIcpKv {
    async fn collection(&self, name: &str) -> Result<impl MuKvCollection, MuError> {
        //use has_collection to check if it exists and if not create it
        let exists: Result<(bool,), _> =
            ic_cdk::call(*KV_PRINCIPAL, "has_collection", (name.to_owned(),))
                .await
                .map_err(|_| MuError::InternalError);

        if !exists?.0 {
            let res: Result<(), _> =
                ic_cdk::call(*KV_PRINCIPAL, "create_collection", (name.to_owned(),)).await;
            if res.is_err() {
                return Err(MuError::InternalError);
            }
        }

        Ok(MuIcpKvCollection {
            name: name.to_owned(),
        })
    }
}

impl MuKvCollection for MuIcpKvCollection {
    async fn query(&self, key: &str) -> Result<Option<String>, MuError> {
        let result: Result<(String,), _> = ic_cdk::call(
            *KV_PRINCIPAL,
            "query_collection",
            (self.name.to_owned(), key.to_owned()),
        )
        .await;
        result
            .map_err(|_| MuError::InternalError)
            .map(|(value,)| Some(value))
    }

    async fn keys(&self) -> Result<Vec<String>, MuError> {
        let result: Result<(Vec<String>,), _> =
            ic_cdk::call(*KV_PRINCIPAL, "query_all_keys", (self.name.clone(),)).await;
        result
            .map_err(|_| MuError::InternalError)
            .map(|(keys,)| keys)
    }

    async fn values(&self) -> Result<Vec<String>, MuError> {
        let result: Result<(Vec<String>,), _> =
            ic_cdk::call(*KV_PRINCIPAL, "query_all_values", (self.name.clone(),)).await;
        result
            .map_err(|_| MuError::InternalError)
            .map(|(values,)| values)
    }

    async fn update(&self, key: &str, value: Option<&str>) -> Result<(), MuError> {
        let result: Result<(), _> = ic_cdk::call(
            *KV_PRINCIPAL,
            "update_collection",
            (
                self.name.clone(),
                key.to_owned(),
                value.map(|s| s.to_owned()),
            ),
        )
        .await;
        result.map_err(|_| MuError::InternalError)
    }

    async fn delete(&self) -> Result<(), MuError> {
        let result: Result<(), _> =
            ic_cdk::call(*KV_PRINCIPAL, "delete_collection", (self.name.clone(),)).await;
        result.map_err(|_| MuError::InternalError)
    }
}
