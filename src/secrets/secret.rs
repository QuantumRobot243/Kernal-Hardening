use secrecy::{Secret, ExposeSecret};
use std::sync::{Mutex, Arc, Weak};
use lazy_static::lazy_static;
use zeroize::Zeroize;

lazy_static! {
    static ref SECRET_REGISTRY: Mutex<Vec<Box<dyn Fn() + Send + Sync>>> = Mutex::new(Vec::new());
}

pub struct SecureSecret {
    inner: Arc<Mutex<Secret<[u8; 32]>>>,
}

impl SecureSecret {
    pub fn new(initial_data: [u8; 32]) -> Self {
        let inner = Arc::new(Mutex::new(Secret::new(initial_data)));
        let weak_inner = Arc::downgrade(&inner);

        if let Ok(mut registry) = SECRET_REGISTRY.lock() {
            registry.push(Box::new(move || {
                if let Some(strong_inner) = weak_inner.upgrade() {
                    if let Ok(mut secret) = strong_inner.lock() {
                        let secret_mut = secret.expose_secret();
                        let ptr = secret_mut.as_ptr() as *mut u8;
                        unsafe {
                            std::ptr::write_bytes(ptr, 0, 32);
                        }
                    }
                }
            }));
        }

        Self { inner }
    }

    pub fn expose(&self) -> [u8; 32] {
        *self.inner.lock().unwrap().expose_secret()
    }
}

pub fn wipe_all_registered_secrets() {
    if let Ok(mut registry) = SECRET_REGISTRY.lock() {
        for wipe_fn in registry.iter() {
            wipe_fn();
        }
        registry.clear();
    }
}
