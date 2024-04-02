use napi_derive::napi;
use std::collections::HashSet;

#[napi]
pub struct UnorderedSet {
    inner: HashSet<String>,
}

#[napi]
impl UnorderedSet {
    #[napi(constructor)]
    pub fn new() -> Self {
        UnorderedSet {
            inner: HashSet::new(),
        }
    }

    #[napi]
    pub fn insert(&mut self, value: String) {
        self.inner.insert(value);
    }

    #[napi]
    pub fn has(&self, value: String) -> bool {
        self.inner.contains(&value)
    }

    #[napi]
    pub fn iterate(&self) -> Vec<String> {
        self.inner.iter().cloned().collect()
    }
}
