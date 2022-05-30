use crate::{StableHash, StableHasher};
use arrayvec::ArrayString;
use bytemuck::{Pod, TransparentWrapper, Zeroable};
use core::{
    borrow::Borrow,
    fmt::{self, Debug, Display, LowerHex, UpperHex},
};

const HASH_LEN: usize = 32;
const HEX_UPPER: [u8; 16] = *b"0123456789ABCDEF";
const HEX_LOWER: [u8; 16] = *b"0123456789abcdef";

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Pod, Zeroable, TransparentWrapper)]
#[repr(transparent)]
pub struct StablyHashed {
    hash: [u8; HASH_LEN],
}

impl StablyHashed {
    #[inline]
    pub const fn new(hash: [u8; HASH_LEN]) -> Self {
        Self { hash }
    }

    #[inline]
    pub const fn into_inner(self) -> [u8; HASH_LEN] {
        self.hash
    }

    #[inline]
    fn to_hex(self, hex: [u8; 16]) -> ArrayString<{ HASH_LEN * 2 }> {
        let mut output = ArrayString::new();
        for elem in self.hash {
            output.push(hex[(elem >> 4) as usize] as char);
            output.push(hex[(elem & 0xF) as usize] as char);
        }

        output
    }
}

impl StableHash for StablyHashed {
    #[inline]
    fn stable_hash<H>(&self, state: &mut H)
    where
        H: StableHasher,
    {
        state.write(&self.hash);
    }

    #[inline]
    fn stable_hash_slice<H>(slice: &[Self], state: &mut H)
    where
        Self: Sized,
        H: StableHasher,
    {
        let bytes = bytemuck::cast_slice::<Self, u8>(slice);
        state.write(bytes);
    }
}

impl AsRef<[u8]> for StablyHashed {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        &self.hash
    }
}

impl Borrow<[u8]> for StablyHashed {
    #[inline]
    fn borrow(&self) -> &[u8] {
        &self.hash
    }
}

impl Debug for StablyHashed {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        LowerHex::fmt(self, f)
    }
}

impl Display for StablyHashed {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        LowerHex::fmt(self, f)
    }
}

impl LowerHex for StablyHashed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            f.write_str("0x")?;
        }

        f.write_str(&*self.to_hex(HEX_LOWER))
    }
}

impl UpperHex for StablyHashed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            f.write_str("0x")?;
        }

        f.write_str(&*self.to_hex(HEX_UPPER))
    }
}
