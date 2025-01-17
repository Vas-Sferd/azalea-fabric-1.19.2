use ahash::AHashMap;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// An NBT value.
#[derive(Clone, Debug, PartialEq, Default)]
#[repr(u8)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(untagged))]
pub enum Tag {
    #[default]
    End = 0,
    Byte(i8) = 1,
    Short(i16) = 2,
    Int(i32) = 3,
    Long(i64) = 4,
    Float(f32) = 5,
    Double(f64) = 6,
    ByteArray(Vec<u8>) = 7,
    String(String) = 8,
    List(Vec<Tag>) = 9,
    Compound(AHashMap<String, Tag>) = 10,
    IntArray(Vec<i32>) = 11,
    LongArray(Vec<i64>) = 12,
}

impl Tag {
    /// Get the numerical ID of the tag type.
    #[inline]
    pub fn id(&self) -> u8 {
        // SAFETY: Because `Self` is marked `repr(u8)`, its layout is a `repr(C)`
        // `union` between `repr(C)` structs, each of which has the `u8`
        // discriminant as its first field, so we can read the discriminant
        // without offsetting the pointer.
        unsafe { *<*const _>::from(self).cast::<u8>() }
    }

    /// If the type is a byte, return the [`i8`].
    #[inline]
    pub fn as_byte(&self) -> Option<&i8> {
        if let Tag::Byte(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// If the type is a short, return the [`i16`].
    #[inline]
    pub fn as_short(&self) -> Option<&i16> {
        if let Tag::Short(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// If the type is an int, return the [`i32`].
    #[inline]
    pub fn as_int(&self) -> Option<&i32> {
        if let Tag::Int(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// If the type is a long, return the [`i64`].
    #[inline]
    pub fn as_long(&self) -> Option<&i64> {
        if let Tag::Long(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// If the type is a float, return the [`f32`].
    #[inline]
    pub fn as_float(&self) -> Option<&f32> {
        if let Tag::Float(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// If the type is a double, return the [`f64`].
    #[inline]
    pub fn as_double(&self) -> Option<&f64> {
        if let Tag::Double(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// If the type is a string, return the [`str`].
    #[inline]
    pub fn as_string(&self) -> Option<&str> {
        if let Tag::String(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// If the type is a compound, return the `AHashMap<String, Tag>`.
    #[inline]
    pub fn as_compound(&self) -> Option<&AHashMap<String, Tag>> {
        if let Tag::Compound(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// If the type is a bytearray, return the `[u8]`.
    #[inline]
    pub fn as_bytearray(&self) -> Option<&[u8]> {
        if let Tag::ByteArray(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// If the type is an intarray, return the `Vec<i32>`.
    #[inline]
    pub fn as_intarray(&self) -> Option<&Vec<i32>> {
        if let Tag::IntArray(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// If the type is a longarray, return the `Vec<i64>`.
    #[inline]
    pub fn as_longarray(&self) -> Option<&Vec<i64>> {
        if let Tag::LongArray(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// If the type is a list, return the `[Tag]`.
    #[inline]
    pub fn as_list(&self) -> Option<&[Tag]> {
        if let Tag::List(v) = self {
            Some(v)
        } else {
            None
        }
    }
}
