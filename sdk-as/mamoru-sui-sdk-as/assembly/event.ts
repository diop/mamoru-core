import { readMemory, unpackValues } from "@mamoru-ai/mamoru-sdk-as/assembly/util";
import { _mamoru_get_events } from "./imports";
import { Decoder } from "@wapc/as-msgpack/assembly";

export class Event {
    public readonly txSeq: u64
    public readonly packageId: string
    public readonly transactionModule: string
    public readonly sender: string
    public readonly typ: string
    public readonly contents: Uint8Array

    private constructor(tx_seq: u64, package_id: string, transaction_module: string, sender: string, typ: string, contents: Uint8Array) {
        this.txSeq = tx_seq
        this.packageId = package_id
        this.transactionModule = transaction_module
        this.sender = sender
        this.typ = typ
        this.contents = contents
    }

    public static loadAll(): Event[] {
        let ptr_len = unpackValues(_mamoru_get_events());

        return Event.fromHost(ptr_len[0], ptr_len[1]);
    }

    private static fromHost(ptr: u32, len: u32): Event[] {
        const data = readMemory(ptr, len);
        const decoder = new Decoder(data.buffer);

        return decoder.readArray<Event>((decoder: Decoder) => {
            // consume array size (we can't parse data otherwise)
            let _ = decoder.readArraySize();

            let tx_seq = decoder.readUInt64();
            let package_id = decoder.readString();
            let transaction_module = decoder.readString();
            let sender = decoder.readString();
            let typ = decoder.readString();
            let contents = Uint8Array.wrap(decoder.readByteArray());

            return new Event(tx_seq, package_id, transaction_module, sender, typ, contents);
        })
    }
}
