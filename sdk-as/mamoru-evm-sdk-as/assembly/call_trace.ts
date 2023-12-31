import { _mamoru_get_call_traces } from "./imports";
import { msgPackReadUint8Array, readMemory, unpackValues } from "@mamoru-ai/mamoru-sdk-as/assembly/util";
import { Decoder } from "@wapc/as-msgpack/assembly";
import { TxInput } from "./tx_input";

export class CallTrace {
    public readonly seq: u32
    public readonly txIndex: u32
    public readonly blockIndex: u64
    public readonly depth: u32
    public readonly typ: string
    public readonly from: string
    public readonly to: string
    public readonly value: u64
    public readonly gasLimit: u64
    public readonly gasUsed: u64
    public readonly input: TxInput

    private constructor(
        seq: u32,
        tx_index: u32,
        block_index: u64,
        depth: u32,
        typ: string,
        from: string,
        to: string,
        value: u64,
        gas_limit: u64,
        gas_used: u64,
        input: TxInput
    ) {
        this.seq = seq
        this.txIndex = tx_index
        this.blockIndex = block_index
        this.depth = depth
        this.typ = typ
        this.from = from
        this.to = to
        this.value = value
        this.gasLimit = gas_limit
        this.gasUsed = gas_used
        this.input = input
    }

    public static loadAll(): CallTrace[] {
        let ptr_len = unpackValues(_mamoru_get_call_traces());

        return CallTrace.fromHost(ptr_len[0], ptr_len[1]);
    }

    private static fromHost(ptr: u32, len: u32): CallTrace[] {
        const decoder = new Decoder(readMemory(ptr, len).buffer);

        return decoder.readArray<CallTrace>((decoder: Decoder) => {
            // consume array size (we can't parse data otherwise)
            let _ = decoder.readArraySize();

            const seq = decoder.readUInt32();
            const tx_index = decoder.readUInt32();
            const block_index = decoder.readUInt64();
            const depth = decoder.readUInt32();
            const typ = decoder.readString();
            const from = decoder.readString();
            const to = decoder.readString();
            const value = decoder.readUInt64();
            const gas_limit = decoder.readUInt64();
            const gas_used = decoder.readUInt64();
            const input = new TxInput(msgPackReadUint8Array(decoder));

            return new CallTrace(
                seq,
                tx_index,
                block_index,
                depth,
                typ,
                from,
                to,
                value,
                gas_limit,
                gas_used,
                input
            );
        });
    }
}
