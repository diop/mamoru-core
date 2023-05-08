import { _mamoru_get_blocks } from "./imports";
import { readMemory, unpackValues } from "@mamoru-ai/mamoru-sdk-as/assembly/util";
import { Decoder } from "@wapc/as-msgpack/assembly";

export class Block {
    public readonly hash: string
    public readonly epoch: u64
    public readonly timestampUsecs: u64

    private constructor(hash: string, epoch: u64, timestamp_usecs: u64) {
        this.hash = hash
        this.epoch = epoch
        this.timestampUsecs = timestamp_usecs
    }

    public static loadAll(): Block[] {
        let ptr_len = unpackValues(_mamoru_get_blocks());

        return Block.fromHost(ptr_len[0], ptr_len[1]);
    }

    private static fromHost(ptr: u32, len: u32): Block[] {
        const decoder = new Decoder(readMemory(ptr, len).buffer);

        return decoder.readArray<Block>((decoder: Decoder) => {
            // consume array size (we can't parse data otherwise)
            let _ = decoder.readArraySize();

            let hash = decoder.readString();
            let epoch = decoder.readUInt64();
            let timestamp_usecs = decoder.readUInt64();

            return new Block(hash, epoch, timestamp_usecs);
        });
    }

}