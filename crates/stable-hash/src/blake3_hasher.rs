use crate::{StableHasher, StablyHashed};
use blake3::Hasher;

const KEY_DERIVATION_CONTEXT: &str =
    "Differential Datalog 2022-05-09 09:19:16 Content Addressed Hashing";

#[derive(Debug, Clone)]
pub struct Blake3Hasher {
    hasher: Hasher,
}

impl Blake3Hasher {
    #[inline]
    pub fn new() -> Self {
        Self {
            hasher: Hasher::new_derive_key(KEY_DERIVATION_CONTEXT),
        }
    }
}

impl core::hash::Hasher for Blake3Hasher {
    fn finish(&self) -> u64 {
        panic!("use `StableHasher::finalize()` instead of `Hasher::finish()`")
    }

    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        self.hasher.update(bytes);
    }

    #[inline]
    fn write_u8(&mut self, i: u8) {
        self.write(&[i]);
    }

    #[inline]
    fn write_u16(&mut self, i: u16) {
        self.write(&i.to_le_bytes());
    }

    #[inline]
    fn write_u32(&mut self, i: u32) {
        self.write(&i.to_le_bytes());
    }

    #[inline]
    fn write_u64(&mut self, i: u64) {
        self.write(&i.to_le_bytes());
    }

    #[inline]
    fn write_u128(&mut self, i: u128) {
        self.write(&i.to_le_bytes());
    }

    #[inline]
    fn write_usize(&mut self, i: usize) {
        // Always treat usize as u64s so we get the same
        // behavior across 32 and 64 bit platforms
        self.write(&(i as u64).to_le_bytes());
    }

    // Treat signed numbers as unsigned ones so that things are consistent
    // across platforms
    #[inline]
    fn write_i8(&mut self, i: i8) {
        self.write(&[(i as u8)]);
    }

    #[inline]
    fn write_i16(&mut self, i: i16) {
        self.write(&(i as u16).to_le_bytes());
    }

    #[inline]
    fn write_i32(&mut self, i: i32) {
        self.write(&(i as u32).to_le_bytes());
    }

    #[inline]
    fn write_i64(&mut self, i: i64) {
        self.write(&(i as u64).to_le_bytes());
    }

    #[inline]
    fn write_i128(&mut self, i: i128) {
        self.write(&(i as u128).to_le_bytes());
    }

    #[inline]
    fn write_isize(&mut self, i: isize) {
        // Always treat isize as a 64-bit number so we get the same results on 32 and 64 bit
        // platforms. This is important for symbol hashes when cross compiling,
        // for example. Sign extending here is preferable as it means that the
        // same negative number hashes the same on both 32 and 64 bit platforms.
        let value = i as u64;

        // `isize` values often seem to have a small (positive) numeric value in practice.
        // To exploit this, if the value is small, we will hash a smaller amount of bytes.
        // However, we cannot just skip the leading zero bytes, as that would produce the same hash
        // e.g. if you hash two values that have the same bit pattern when they are swapped.
        // See https://github.com/rust-lang/rust/pull/93014 for context.
        //
        // Therefore, we employ the following strategy:
        // 1) When we encounter a value that fits within a single byte (the most common case), we
        // hash just that byte. This is the most common case that is being optimized. However, we do
        // not do this for the value 0xFF, as that is a reserved prefix (a bit like in UTF-8).
        // 2) When we encounter a larger value, we hash a "marker" 0xFF and then the corresponding
        // 8 bytes. Since this prefix cannot occur when we hash a single byte, when we hash two
        // `isize`s that fit within a different amount of bytes, they should always produce a different
        // byte stream for the hasher.
        if value < 0xFF {
            self.write_u8(value as u8);
        } else {
            self.write_u8(0xFF);
            self.write(&value.to_le_bytes());
        }
    }
}

impl StableHasher for Blake3Hasher {
    #[inline]
    fn finalize(&mut self) -> StablyHashed {
        let hash = StablyHashed::new(*self.hasher.finalize().as_bytes());
        self.hasher.reset();
        hash
    }
}

impl Default for Blake3Hasher {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
