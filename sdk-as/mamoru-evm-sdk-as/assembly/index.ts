import { Transaction } from "./tx";
import { Event } from "./event";
import { CallTrace } from "./call_trace";
import { Block } from "./block";
import { Value } from "./value";
import {TxInput} from "./tx_input";

export {
    Transaction,
    TxInput,
    Value,
    Event,
    CallTrace,
    Block,
}

export class EvmCtx {
    private _block: Block | null
    private _txs: Transaction[] | null
    private _events: Event[] | null
    private _callTraces: CallTrace[] | null

    private constructor() {
        this._block = null
        this._txs = null
        this._events = null
        this._callTraces = null
    }

    public static load(): EvmCtx {
        return new EvmCtx()
    }

    public get block(): Block {
        if (this._block == null) {
            const blocks = Block.loadAll();

            this._block = blocks[0]
        }

        return this._block!
    }

    public get txs(): Transaction[] {
        if (this._txs == null) {
            this._txs = Transaction.loadAll()
        }

        return this._txs!
    }

    public get events(): Event[] {
        if (this._events == null) {
            this._events = Event.loadAll()
        }

        return this._events!
    }

    public get callTraces(): CallTrace[] {
        if (this._callTraces == null) {
            this._callTraces = CallTrace.loadAll()
        }

        return this._callTraces!
    }
}
