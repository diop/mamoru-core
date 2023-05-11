import { _mamoru_get_transactions } from "./imports";
import { readMemory, unpackValues } from "@mamoru-ai/mamoru-sdk-as/assembly/util";
import { Decoder } from "@wapc/as-msgpack/assembly";
import { AptosCtx, CallTrace } from "./index";

export class Transaction {
    public readonly seq: u64
    public readonly blockHash: string
    public readonly hash: string
    public readonly eventRootHash: string
    public readonly stateChangeHash: string
    public readonly gasUsed: u64
    public readonly maxGasAmount: u64
    public readonly gasUnitPrice: u64
    public readonly expirationTimestampSecs: u64
    public readonly status: u64
    public readonly sender: string
    public readonly sequenceNumber: u64

    private _ctx: AptosCtx
    private _callTraces: CallTrace[] | null

    private constructor(
        ctx: AptosCtx,
        seq: u64,
        block_hash: string,
        hash: string,
        event_root_hash: string,
        state_change_hash: string,
        gas_used: u64,
        max_gas_amount: u64,
        gas_unit_price: u64,
        expiration_timestamp_secs: u64,
        status: u64,
        sender: string,
        sequence_number: u64
    ) {
        this._ctx = ctx
        this._callTraces = null

        this.seq = seq
        this.blockHash = block_hash
        this.hash = hash
        this.eventRootHash = event_root_hash
        this.stateChangeHash = state_change_hash
        this.gasUsed = gas_used
        this.maxGasAmount = max_gas_amount
        this.gasUnitPrice = gas_unit_price
        this.expirationTimestampSecs = expiration_timestamp_secs
        this.status = status
        this.sender = sender
        this.sequenceNumber = sequence_number
    }

    /// All call traces in the current transaction
    public get callTraces(): CallTrace[] {
        if (this._callTraces == null) {
            let callTraces = new Array<CallTrace>();
            for (let i = 0; i < this._ctx.callTraces.length; i++) {
                const trace = this._ctx.callTraces[i];

                if (trace.txSeq == this.seq) {
                    callTraces.push(trace);
                }
            }

            this._callTraces = callTraces
        }

        return this._callTraces!
    }

    public static loadAll(ctx: AptosCtx): Transaction[] {
        let ptr_len = unpackValues(_mamoru_get_transactions());

        return Transaction.fromHost(ctx, ptr_len[0], ptr_len[1]);
    }

    private static fromHost(ctx: AptosCtx, ptr: u32, len: u32): Transaction[] {
        const decoder = new Decoder(readMemory(ptr, len).buffer);

        let txs = new Array<Transaction>();
        let size = decoder.readArraySize();

        for (let i: u32 = 0; i < size; i++) {
            // consume array size (we can't parse data otherwise)
            let _ = decoder.readArraySize();

            let seq = decoder.readUInt64();
            let block_hash = decoder.readString();
            let hash = decoder.readString();
            let event_root_hash = decoder.readString();
            let state_change_hash = decoder.readString();
            let gas_used = decoder.readUInt64();
            let max_gas_amount = decoder.readUInt64();
            let gas_unit_price = decoder.readUInt64();
            let expiration_timestamp_secs = decoder.readUInt64();
            let status = decoder.readUInt64();
            let sender = decoder.readString();
            let sequence_number = decoder.readUInt64();

            txs.push(new Transaction(
                ctx,
                seq,
                block_hash,
                hash,
                event_root_hash,
                state_change_hash,
                gas_used,
                max_gas_amount,
                gas_unit_price,
                expiration_timestamp_secs,
                status,
                sender,
                sequence_number
            ));
        }

        return txs;

    }
}