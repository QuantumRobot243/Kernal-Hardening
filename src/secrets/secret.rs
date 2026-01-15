use zeroize::Zeroize;

pub struct Secret {
    data: [u8; 32],
}

impl Secret {
    pub fn new() -> Self {
        Secret { data: [42u8; 32] }
    }

    pub fn as_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }
}

impl Drop for Secret {
    fn drop(&mut self) {
        self.data.zeroize();
    }
}
