use zeroize::Zeroize;
use std::sync::Mutex;
use lazy_static::lazy_static;

struct SendPtr(*mut u8);
unsafe impl Send for SendPtr {}
unsafe impl Sync for SendPtr {}

lazy_static! {
    static ref SECRET_REGISTRY: Mutex<Vec<SendPtr>> = Mutex::new(Vec::new());
}

#[derive(Zeroize)]
pub struct Secret {
    pub data: Box<[u8; 32]>,
}

impl Secret {
    pub fn new() -> Self {
        let mut secret = Secret {
            data: Box::new([42u8; 32])
        };

        if let Ok(mut registry) = SECRET_REGISTRY.lock() {
            registry.push(SendPtr(secret.data.as_mut_ptr()));
        }

        secret
    }
}

impl Drop for Secret {
    fn drop(&mut self) {
        let my_ptr = self.data.as_mut_ptr();

        if let Ok(mut registry) = SECRET_REGISTRY.lock() {
            registry.retain(|ptr_wrapper| ptr_wrapper.0 != my_ptr);
        }

        self.data.zeroize();
    }
}

pub fn wipe_all_registered_secrets() {
    if let Ok(mut registry) = SECRET_REGISTRY.lock() {
        for ptr_wrapper in registry.iter() {
            unsafe {
                let slice = std::slice::from_raw_parts_mut(ptr_wrapper.0, 32);
                slice.zeroize();
            }
        }
        registry.clear();
    }
}
