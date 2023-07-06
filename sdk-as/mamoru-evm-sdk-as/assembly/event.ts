import { _mamoru_get_events } from "./imports";
import { msgPackReadUint8Array, readMemory, unpackValues } from "@mamoru-ai/mamoru-sdk-as/assembly/util";
import { Decoder } from "@wapc/as-msgpack/assembly";

export class Event {
    public readonly index: u32
    public readonly txIndex: u32
    public readonly txHash: string
    public readonly blockNumber: u64
    public readonly blockHash: string
    public readonly address: string
    public readonly topic0: Uint8Array
    public readonly topic1: Uint8Array
    public readonly topic2: Uint8Array
    public readonly topic3: Uint8Array
    public readonly topic4: Uint8Array
    public readonly data: Uint8Array

    private constructor(
        index: u32,
        tx_index: u32,
        tx_hash: string,
        block_number: u64,
        block_hash: string,
        address: string,
        topic0: Uint8Array,
        topic1: Uint8Array,
        topic2: Uint8Array,
        topic3: Uint8Array,
        topic4: Uint8Array,
        data: Uint8Array
    ) {
        this.index = index
        this.txIndex = tx_index
        this.txHash = tx_hash
        this.blockNumber = block_number
        this.blockHash = block_hash
        this.address = address
        this.topic0 = topic0
        this.topic1 = topic1
        this.topic2 = topic2
        this.topic3 = topic3
        this.topic4 = topic4
        this.data = data
    }

    public static loadAll(): Event[] {
        let ptr_len = unpackValues(_mamoru_get_events());

        return Event.fromHost(ptr_len[0], ptr_len[1]);
    }

    private static fromHost(ptr: u32, len: u32): Event[] {
        const decoder = new Decoder(readMemory(ptr, len).buffer);

        return decoder.readArray<Event>((decoder: Decoder) => {
            // consume array size (we can't parse data otherwise)
            let _ = decoder.readArraySize();

            let index = decoder.readUInt32();
            let tx_index = decoder.readUInt32();
            let tx_hash = decoder.readString();
            let block_number = decoder.readUInt64();
            let block_hash = decoder.readString();
            let address = decoder.readString();
            let topic0 = msgPackReadUint8Array(decoder);
            let topic1 = msgPackReadUint8Array(decoder);
            let topic2 = msgPackReadUint8Array(decoder);
            let topic3 = msgPackReadUint8Array(decoder);
            let topic4 = msgPackReadUint8Array(decoder);
            let data = msgPackReadUint8Array(decoder);

            return new Event(index, tx_index, tx_hash, block_number, block_hash, address, topic0, topic1, topic2, topic3, topic4, data);
        });
    }
}
