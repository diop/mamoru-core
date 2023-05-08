import { _mamoru_get_transactions } from "./imports";
import { readMemory, unpackValues } from "@mamoru-ai/mamoru-sdk-as/assembly/util";
import { Decoder } from "@wapc/as-msgpack/assembly";

export class Transaction {
    public readonly txIndex: u32
    public readonly txHash: string
    public readonly typ: u8
    public readonly nonce: u64
    public readonly status: u64
    public readonly blockIndex: u64
    public readonly from: string
    public readonly to: string | null
    public readonly value: u64
    public readonly fee: u64
    public readonly gasPrice: u64
    public readonly gasLimit: u64
    public readonly gasUsed: u64
    public readonly input: Uint8Array
    public readonly size: f64

    private constructor(
        tx_index: u32,
        tx_hash: string,
        typ: u8,
        nonce: u64,
        status: u64,
        block_index: u64,
        from: string,
        to: string | null,
        value: u64,
        fee: u64,
        gas_price: u64,
        gas_limit: u64,
        gas_used: u64,
        input: Uint8Array,
        size: f64
    ) {
        this.txIndex = tx_index
        this.txHash = tx_hash
        this.typ = typ
        this.nonce = nonce
        this.status = status
        this.blockIndex = block_index
        this.from = from
        this.to = to
        this.value = value
        this.fee = fee
        this.gasPrice = gas_price
        this.gasLimit = gas_limit
        this.gasUsed = gas_used
        this.input = input
        this.size = size
    }

    public static loadAll(): Transaction[] {
        let ptr_len = unpackValues(_mamoru_get_transactions());

        return Transaction.fromHost(ptr_len[0], ptr_len[1]);
    }

    private static fromHost(ptr: u32, len: u32): Transaction[] {
        const decoder = new Decoder(readMemory(ptr, len).buffer);

        return decoder.readArray<Transaction>((decoder: Decoder) => {
            // consume array size (we can't parse data otherwise)
            let _ = decoder.readArraySize();

            let tx_index = decoder.readUInt32();
            let tx_hash = decoder.readString();
            let typ = decoder.readUInt8();
            let nonce = decoder.readUInt64();
            let status = decoder.readUInt64();
            let block_index = decoder.readUInt64();
            let from = decoder.readString();

            let to: string | null;
            if (decoder.isNextNil()) {
                to = null;
            } else {
                to = decoder.readString();
            }

            let value = decoder.readUInt64();
            let fee = decoder.readUInt64();
            let gas_price = decoder.readUInt64();
            let gas_limit = decoder.readUInt64();
            let gas_used = decoder.readUInt64();
            let input = Uint8Array.wrap(decoder.readByteArray());
            let size = decoder.readFloat64();

            return new Transaction(
                tx_index,
                tx_hash,
                typ,
                nonce,
                status,
                block_index,
                from,
                to,
                value,
                fee,
                gas_price,
                gas_limit,
                gas_used,
                input,
                size
            );
        });
    }

}