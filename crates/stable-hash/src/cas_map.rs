use crate::{Blake3Hasher, StableHash, StableHasher, StablyHashed};
use core::fmt::{self, Debug};
use qp_trie::Trie;

/// A content-addressed map
#[derive(Clone)]
pub struct CasMap<V> {
    trie: Trie<StablyHashed, V>,
    hasher: Blake3Hasher,
}

impl<V> CasMap<V> {
    /// Create a new content addressed map
    #[inline]
    pub fn new() -> Self {
        Self {
            trie: Trie::new(),
            hasher: Blake3Hasher::new(),
        }
    }

    /// Remove all elements from the current map
    #[inline]
    pub fn clear(&mut self) {
        self.trie.clear();
    }
}

impl<V> CasMap<V>
where
    V: StableHash,
{
    /// Insert a new element into the current map
    #[inline]
    pub fn insert(&mut self, value: V) -> (StablyHashed, Option<V>) {
        let hash = self.hash_value(&value);
        let replaced = self.trie.insert(hash, value);
        (hash, replaced)
    }

    /// Get an element by its hash from the current map,
    /// returning `None` if it doesn't exist
    #[inline]
    pub fn get(&self, hash: StablyHashed) -> Option<&V> {
        self.trie.get(&hash)
    }

    /// Get the hash of the given value
    #[inline]
    pub fn hash_value(&mut self, value: &V) -> StablyHashed {
        self.hasher.hash_one(value)
    }
}

impl<V> Default for CasMap<V> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<V> Debug for CasMap<V>
where
    V: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map().entries(self.trie.iter()).finish()
    }
}
