import { readMemory, unpackValues } from "@mamoru-ai/mamoru-sdk-as/assembly/util";
import { _mamoru_get_events } from "./imports";
import { Decoder } from "@wapc/as-msgpack/assembly";

export class Event {
    public readonly txSeq: u64
    public readonly key: string
    public readonly sequenceNumber: u64
    public readonly typ: string
    public readonly data: Uint8Array

    private constructor(tx_seq: u64, key: string, sequence_number: u64, typ: string, data: Uint8Array) {
        this.txSeq = tx_seq
        this.key = key
        this.sequenceNumber = sequence_number
        this.typ = typ
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

            let tx_seq = decoder.readUInt64();
            let key = decoder.readString();
            let sequence_number = decoder.readUInt64();
            let typ = decoder.readString();
            let data = Uint8Array.wrap(decoder.readByteArray());

            return new Event(tx_seq, key, sequence_number, typ, data);
        });
    }
}
