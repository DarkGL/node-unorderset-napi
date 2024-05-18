use napi_derive::napi;
use tinyset::SetU32;

#[napi]
pub struct UnorderedSet {
    inner: SetU32,
}

#[napi]
impl UnorderedSet {
    #[napi(constructor)]
    pub fn new() -> Self {
        UnorderedSet {
            inner: SetU32::new(),
        }
    }

    #[napi]
    pub fn insert(&mut self, value: u32) {
        self.inner.insert(value);
    }

    #[napi]
    pub fn has(&self, value: u32) -> bool {
        self.inner.contains(value)
    }
}
