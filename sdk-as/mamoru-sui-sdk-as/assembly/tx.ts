import { Decoder } from "@wapc/as-msgpack/assembly"
import { _mamoru_get_transactions } from "./imports";
import { readMemory, unpackValues } from "@mamoru-ai/mamoru-sdk-as/assembly/util";

export class Transaction {
    public readonly seq: u64
    public readonly digest: string
    public readonly time: i64
    public readonly gasUsed: u64
    public readonly gasComputationCost: u64
    public readonly gasStorageCost: u64
    public readonly gasBudget: u64
    public readonly sender: string
    public readonly kind: string

    private constructor(
        seq: u64,
        digest: string,
        time: i64,
        gas_used: u64,
        gas_computation_cost: u64,
        gas_storage_cost: u64,
        gas_budget: u64,
        sender: string,
        kind: string,
    ) {
        this.seq = seq;
        this.digest = digest;
        this.time = time;
        this.gasUsed = gas_used;
        this.gasComputationCost = gas_computation_cost;
        this.gasStorageCost = gas_storage_cost;
        this.gasBudget = gas_budget;
        this.sender = sender;
        this.kind = kind;
    }

    public static loadAll(): Transaction[] {
        let ptr_len = unpackValues(_mamoru_get_transactions());

        return Transaction.fromHost(ptr_len[0], ptr_len[1]);
    }

    private static fromHost(ptr: u32, len: u32): Transaction[] {
        const data = readMemory(ptr, len);
        const decoder = new Decoder(data.buffer);

        return decoder.readArray<Transaction>((decoder: Decoder) => {
            // consume array size (we can't parse data otherwise)
            let _ = decoder.readArraySize();

            let seq = decoder.readUInt64();
            let digest = decoder.readString();
            let time = decoder.readInt64();
            let gas_used = decoder.readUInt64();
            let gas_computation_cost = decoder.readUInt64();
            let gas_storage_cost = decoder.readUInt64();
            let gas_budget = decoder.readUInt64();
            let sender = decoder.readString();
            let kind = decoder.readString();

            return new Transaction(
                seq,
                digest,
                time,
                gas_used,
                gas_computation_cost,
                gas_storage_cost,
                gas_budget,
                sender,
                kind,
            )
        })
    }
}
