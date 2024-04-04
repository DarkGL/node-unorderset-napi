use napi_derive::napi;
use tinyset::Set64;
use napi::JsBigInt;

#[napi]
pub struct UnorderedSet {
    inner: Set64<i64>,
}

#[napi]
impl UnorderedSet {
    #[napi(constructor)]
    pub fn new() -> Self {
        UnorderedSet {
            inner: Set64::new(),
        }
    }

    #[napi]
    pub fn insert(&mut self, value: JsBigInt ) {
        self.inner.insert(value.get_i64().unwrap().0);
    }

    #[napi]
    pub fn has(&self, value: JsBigInt) -> bool {
        self.inner.contains(value.get_i64().unwrap().0)
    }
}
