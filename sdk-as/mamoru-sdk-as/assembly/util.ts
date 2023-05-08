/**
 * Reads memory from the specified pointer for the specified length.
 *
 * @param ptr
 * @param length
 */
export function readMemory(ptr: u32, length: u32): Uint8Array {
    let array = new Uint8Array(length);
    for (let i: u32 = 0; i < length; i++) {
        array[i] = load<u8>(ptr + i);
    }
    return array;
}

/**
 * Unpacks u64 into two u32 values: a pointer from the upper 32 bits and a length from the lower 32 bits.
 *
 * @param packed
 */
export function unpackValues(packed: u64): u32[] {
    const ptr: u64 = packed >> 32;   // shift packed right by 32 bits to get upper 32 bits
    const len: u64 = packed & 0xFFFFFFFF;  // mask packed with lower 32 bits to get lower 32 bits

    return [ptr as u32, len as u32];
}
