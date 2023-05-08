use rmp_serde::Serializer;
use serde::{Deserialize, Serialize};

/// Implements the serialization format used for the data passed to the AssemblyScript runtime.
pub fn serialize_data(data: &impl Serialize) -> Vec<u8> {
    let mut buf = Vec::new();

    data.serialize(&mut Serializer::new(&mut buf))
        .expect("BUG: data serialization failed");

    buf
}

pub fn deserialize_data<'a, T: Deserialize<'a>>(
    bytes: &'a [u8],
) -> Result<T, rmp_serde::decode::Error> {
    rmp_serde::from_slice(bytes)
}

/// Pack a pointer and length into a single u64,
/// where the upper 32 bits are the pointer and the lower 32 bits are the length.
///
/// This is used to pass a pointer and length to the AssemblyScript runtime,
/// as it can't handle passing two arguments.
pub fn pack_values(ptr: u32, len: u32) -> u64 {
    ((ptr as u64) << 32) | (len as u64)
}
