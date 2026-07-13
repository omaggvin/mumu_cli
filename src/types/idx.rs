use std::{
    fmt,
    hash::{Hash, Hasher},
    marker::PhantomData,
    str::FromStr,
};

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Marker for MuMu VM slot indices. See [`SlotIndex`].
pub enum Slot {}

/// A MuMu VM slot index. The only way slot numbers enter or leave this crate's API.
pub type SlotIndex = Idx<Slot>;

/// A typed `u32` index. `T` is a phantom marker so indices into different
/// spaces are distinct types and can't be mixed (`Idx<Slot>` today; future
/// id kinds add their own marker + alias).
///
/// Traits are implemented manually so they never require anything of `T`
/// (markers are uninhabited). Serializes transparently as the bare number.
pub struct Idx<T>(u32, PhantomData<T>);

impl<T> Idx<T> {
    pub const fn new(raw: u32) -> Self {
        Self(raw, PhantomData)
    }

    /// The raw number, for wire formats and display only — don't pass it
    /// around as an index.
    pub const fn get(self) -> u32 {
        self.0
    }
}

impl<T> Clone for Idx<T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<T> Copy for Idx<T> {}

impl<T> PartialEq for Idx<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<T> Eq for Idx<T> {}

impl<T> PartialOrd for Idx<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<T> Ord for Idx<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl<T> Hash for Idx<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<T> fmt::Debug for Idx<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Idx").field(&self.0).finish()
    }
}

impl<T> fmt::Display for Idx<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T> From<u32> for Idx<T> {
    fn from(raw: u32) -> Self {
        Self::new(raw)
    }
}

impl<T> FromStr for Idx<T> {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<u32>().map(Self::new)
    }
}

impl<T> Serialize for Idx<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(serializer)
    }
}

impl<'de, T> Deserialize<'de> for Idx<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        u32::deserialize(deserializer).map(Self::new)
    }
}
