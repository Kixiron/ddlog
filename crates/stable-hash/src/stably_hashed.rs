use arrayvec::ArrayString;
use std::fmt::{self, Debug, Display, LowerHex, UpperHex};

const HEX_UPPER: [u8; 16] = *b"0123456789ABCDEF";
const HEX_LOWER: [u8; 16] = *b"0123456789abcdef";

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct StablyHashed {
    hash: [u8; Self::HASH_LEN],
}

impl StablyHashed {
    pub const HASH_LEN: usize = 32;

    #[inline]
    pub const fn new(hash: [u8; Self::HASH_LEN]) -> Self {
        Self { hash }
    }

    #[inline]
    pub const fn into_inner(self) -> [u8; Self::HASH_LEN] {
        self.hash
    }

    #[inline]
    fn to_hex(self, hex: [u8; 16]) -> ArrayString<{ Self::HASH_LEN * 2 }> {
        let mut output = ArrayString::new();
        for elem in self.hash {
            output.push(hex[(elem >> 4) as usize] as char);
            output.push(hex[(elem & 0xF) as usize] as char);
        }

        output
    }
}

impl Debug for StablyHashed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        LowerHex::fmt(self, f)
    }
}

impl Display for StablyHashed {
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
