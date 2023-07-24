use crate::models::caches::Caches;
use std::sync::Arc;

mod example;

#[derive(Clone)]
pub struct Services {
    caches: Caches,
}

impl Services {
    pub fn new(caches: Caches) -> Arc<Self> {
        Arc::new(Self { caches })
    }
}
