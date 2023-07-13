import { Value } from "./value";
import { encode } from "as-base64/assembly"
import { _mamoru_parse_tx_input } from "./imports";
import { readMemory, unpackValues } from "@mamoru-ai/mamoru-sdk-as/assembly";

export class TxInput {
    public readonly data: Uint8Array

    constructor(data: Uint8Array) {
        this.data = data
    }

    // Parse the input data using the provided ABI.
    // Returns null if the input data does not match the ABI.
    // Fails if the ABI is invalid.
    public parse(abi: string): Value[] | null {
        const base64Data = encode(this.data);
        const result = _mamoru_parse_tx_input(abi, base64Data)

        if (result == 0) {
            return null;
        }

        const ptr_len = unpackValues(result);
        const buffer = readMemory(ptr_len[0], ptr_len[1]).buffer;

        return Value.fromBytes(buffer);
    }
}
