use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use bytemuck::{Pod, Zeroable};

macro_rules! impl_int_conversion {
    ($P:ty, $I:ty) => {
        impl $P {
            pub const fn from_primitive(n: $I) -> Self {
                Self(n.to_le_bytes())
            }
        }
        impl From<$I> for $P {
            fn from(n: $I) -> Self {
                Self::from_primitive(n)
            }
        }
        impl From<$P> for $I {
            fn from(pod: $P) -> Self {
                Self::from_le_bytes(pod.0)
            }
        }
    };
}

/// `u32` type that can be used in `Pod`s
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    PartialEq,
    Pod,
    Zeroable,
    BorshDeserialize,
    BorshSerialize,
    BorshSchema,
)]
#[repr(transparent)]
pub struct PodU32(pub [u8; 4]);
impl_int_conversion!(PodU32, u32);

/// `u64` type that can be used in Pods
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    PartialEq,
    Pod,
    Zeroable,
    BorshDeserialize,
    BorshSerialize,
    BorshSchema,
)]
#[repr(transparent)]
pub struct PodU64(pub [u8; 8]);
impl_int_conversion!(PodU64, u64);
