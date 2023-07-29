use std::ops::{Deref, DerefMut};

use rand::{thread_rng, RngCore, SeedableRng};
use rand_chacha::ChaCha8Rng;

pub enum SeedType {
    Random,
    Fixed(u64),
}

impl Default for SeedType {
    fn default() -> Self {
        Self::Random
    }
}

#[derive(Clone, Debug)]
pub struct RngWrapper {
    inner: ChaCha8Rng,
}

impl RngWrapper {
    pub fn new(seed_type: SeedType) -> Self {
        Self {
            inner: match seed_type {
                SeedType::Fixed(value) => ChaCha8Rng::seed_from_u64(value),
                SeedType::Random => ChaCha8Rng::from_rng(thread_rng()).unwrap(),
            },
        }
    }
}

impl DerefMut for RngWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl Deref for RngWrapper {
    type Target = ChaCha8Rng;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl RngCore for RngWrapper {
    fn next_u32(&mut self) -> u32 {
        self.inner.next_u32()
    }

    fn next_u64(&mut self) -> u64 {
        self.inner.next_u64()
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.inner.fill_bytes(dest)
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
        self.inner.try_fill_bytes(dest)
    }
}
