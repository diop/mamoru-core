import { u256 } from "as-bignum/assembly";
import { Decoder } from "@wapc/as-msgpack/assembly";
import { _mamoru_u256_from_str } from "./imports";

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

/**
 * Converts a byte array to a hex string with a 0x prefix.
 *
 * @param bytes
 */
export function bytesToHex(bytes: Uint8Array): string {
    let hex = "0x";

    for (let i = 0; i < bytes.length; i++) {
        hex += bytes[i].toString(16).padStart(2, "0");
    }

    return hex;
}

/**
 * Converts a hex string to a byte array.
 * @param hex
 */
export function hexToBytes(hex: string): Uint8Array {
    // Remove the 0x prefix if it exists.
    if (hex.startsWith("0x")) {
        hex = hex.slice(2);
    }

    const bytes = new Uint8Array(hex.length / 2);

    for (let i: i32 = 0, j: i32 = 0; i < hex.length; i += 2, j++) {
        bytes[j] = u8.parse(hex.substring(i, 2), 16);
    }

    return bytes;
}

/**
 * Parses decimal or hexadecimal string into u256.
 * Use "0x" prefix for hexadecimal strings.
 *
 * @param payload
 */
export function u256FromStr(payload: string): u256 {
    const ptr_len = unpackValues(_mamoru_u256_from_str(payload));
    const bytes = readMemory(ptr_len[0], ptr_len[1]);

    return u256.fromUint8ArrayBE(bytes);
}

/**
 * Converts hex encoded u256 to u256.
 * Does not use imported functions.
 * @param hex
 */
export function hexToU256(hex: string): u256 {
    // Remove the 0x prefix if it exists.
    if (hex.startsWith("0x")) {
        hex = hex.slice(2);
    }

    // Initialize an array to hold the u64 values.
    let result: u64[] = new Array<u64>(4).fill(0);

    // Process each 16-character segment from right to left.
    for (let i = 0; i < 4; i++) {
        // Calculate the start position of the segment.
        let startPos = hex.length - (i + 1) * 16;

        // Extract the segment from the string.
        // If startPos is negative, the segment starts from the beginning of the string.
        let segment = startPos >= 0 ? hex.slice(startPos, startPos + 16) : hex.slice(0, startPos + 16);

        // Convert the segment to a u64 value and store it in the array.
        result[i] = u64.parse(segment, 16);
    }

    return new u256(result[0], result[1], result[2], result[3]);
}

/**
 * Reads an u8 array from the msgpack decoder.
 * The `readByteArray` doesn't work for u8's that are larger than 127 ¯\_(ツ)_/¯
 *
 * @param decoder
 */
export function msgPackReadUint8Array(decoder: Decoder): Uint8Array {
    const size = decoder.readArraySize();
    let result = new Uint8Array(size);

    for (let i: u32 = 0; i < size; i++) {
        result[i] = decoder.readUInt8();
    }

    return result;
}
