use napi_derive::napi;
use tinyset::SetU64;
use xxhash_rust::xxh3::xxh3_64;

#[napi]
pub struct UnorderedSet {
    inner: SetU64,
}

#[napi]
impl UnorderedSet {
    #[napi(constructor)]
    pub fn new() -> Self {
        UnorderedSet {
            inner: SetU64::new(),
        }
    }

    #[napi]
    pub fn insert(&mut self, value: String) {
        let hash = xxh3_64(value.as_bytes());
        self.inner.insert(hash);
    }

    #[napi]
    pub fn has(&self, value: String) -> bool {
        let hash = xxh3_64(value.as_bytes());
        self.inner.contains(hash)
    }
}
