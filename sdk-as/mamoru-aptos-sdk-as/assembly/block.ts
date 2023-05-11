import { _mamoru_get_blocks } from "./imports";
import { readMemory, unpackValues } from "@mamoru-ai/mamoru-sdk-as/assembly/util";
import { Decoder } from "@wapc/as-msgpack/assembly";
import { AptosCtx, Transaction } from "./index";

export class Block {
    public readonly hash: string
    public readonly epoch: u64
    public readonly timestampUsecs: u64

    private _ctx: AptosCtx
    private _txs: Transaction[] | null

    private constructor(ctx: AptosCtx, hash: string, epoch: u64, timestamp_usecs: u64) {
        this._ctx = ctx
        this._txs = null

        this.hash = hash
        this.epoch = epoch
        this.timestampUsecs = timestamp_usecs
    }

    /// All transactions in the current block
    public get txs(): Transaction[] {
        if (this._txs == null) {
            let txs = new Array<Transaction>();
            for (let i = 0; i < this._ctx.txs.length; i++) {
                const tx = this._ctx.txs[i];

                if (tx.blockHash == this.hash) {
                    txs.push(tx);
                }
            }

            this._txs = txs
        }

        return this._txs!
    }

    public static loadAll(ctx: AptosCtx): Block[] {
        let ptr_len = unpackValues(_mamoru_get_blocks());

        return Block.fromHost(ctx, ptr_len[0], ptr_len[1]);
    }

    private static fromHost(ctx: AptosCtx, ptr: u32, len: u32): Block[] {
        const decoder = new Decoder(readMemory(ptr, len).buffer);

        let blocks = new Array<Block>();
        let size = decoder.readArraySize();

        for (let i: u32 = 0; i < size; i++) {
            // consume array size (we can't parse data otherwise)
            let _ = decoder.readArraySize();

            let hash = decoder.readString();
            let epoch = decoder.readUInt64();
            let timestamp_usecs = decoder.readUInt64();

            blocks.push(new Block(ctx, hash, epoch, timestamp_usecs));
        }

        return blocks;
    }

}