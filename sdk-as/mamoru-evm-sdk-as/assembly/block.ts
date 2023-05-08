import { _mamoru_get_blocks } from "./imports";
import { readMemory, unpackValues } from "@mamoru-ai/mamoru-sdk-as/assembly/util";
import { Decoder } from "@wapc/as-msgpack/assembly";

export class Block {
    public readonly blockIndex: u64
    public readonly hash: string
    public readonly parentHash: string
    public readonly stateRoot: string
    public readonly nonce: u64
    public readonly status: string
    public readonly timestamp: u64
    public readonly blockReward: Uint8Array
    public readonly feeRecipient: string
    public readonly totalDifficulty: u64
    public readonly size: f64
    public readonly gasUsed: u64
    public readonly gasLimit: u64

    constructor(
        block_index: u64,
        hash: string,
        parent_hash: string,
        state_root: string,
        nonce: u64,
        status: string,
        timestamp: u64,
        block_reward: Uint8Array,
        fee_recipient: string,
        total_difficulty: u64,
        size: f64,
        gas_used: u64,
        gas_limit: u64
    ) {
        this.blockIndex = block_index
        this.hash = hash
        this.parentHash = parent_hash
        this.stateRoot = state_root
        this.nonce = nonce
        this.status = status
        this.timestamp = timestamp
        this.blockReward = block_reward
        this.feeRecipient = fee_recipient
        this.totalDifficulty = total_difficulty
        this.size = size
        this.gasUsed = gas_used
        this.gasLimit = gas_limit
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

            let block_index = decoder.readUInt64();
            let hash = decoder.readString();
            let parent_hash = decoder.readString();
            let state_root = decoder.readString();
            let nonce = decoder.readUInt64();
            let status = decoder.readString();
            let timestamp = decoder.readUInt64();
            let block_reward = Uint8Array.wrap(decoder.readByteArray());
            let fee_recipient = decoder.readString();
            let total_difficulty = decoder.readUInt64();
            let size = decoder.readFloat64();
            let gas_used = decoder.readUInt64();
            let gas_limit = decoder.readUInt64();

            return new Block(
                block_index,
                hash,
                parent_hash,
                state_root,
                nonce,
                status,
                timestamp,
                block_reward,
                fee_recipient,
                total_difficulty,
                size,
                gas_used,
                gas_limit
            );
        });
    }
}
