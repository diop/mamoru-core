import { Transaction } from "./tx";
import { Event } from "./event";
import { CallTrace, CallTraceArg, CallTraceTypeArg } from "./call_trace";
import { Block } from "./block";

export {
    Transaction,
    Event,
    CallTrace,
    CallTraceArg,
    CallTraceTypeArg,
    Block,
}

export class AptosCtx {
    private _block: Block | null
    private _txs: Transaction[] | null
    private _events: Event[] | null
    private _callTraces: CallTrace[] | null
    private _callTraceTypeArgs: CallTraceTypeArg[] | null
    private _callTraceArgs: CallTraceArg[] | null

    private constructor() {
        this._block = null
        this._txs = null
        this._events = null
        this._callTraces = null
        this._callTraceTypeArgs = null
        this._callTraceArgs = null
    }

    public static load(): AptosCtx {
        return new AptosCtx()
    }

    /// The block of the current context
    public get block(): Block {
        if (this._block == null) {
            const blocks = Block.loadAll(this);

            this._block = blocks[0]
        }

        return this._block!
    }

    /// All transactions in the current context
    public get txs(): Transaction[] {
        if (this._txs == null) {
            this._txs = Transaction.loadAll(this)
        }

        return this._txs!
    }

    /// All events in the current context
    public get events(): Event[] {
        if (this._events == null) {
            this._events = Event.loadAll()
        }

        return this._events!
    }

    /// All call traces in the current context
    public get callTraces(): CallTrace[] {
        if (this._callTraces == null) {
            this._callTraces = CallTrace.loadAll(this)
        }

        return this._callTraces!
    }

    /// All call trace type args in the current context
    public get callTraceTypeArgs(): CallTraceTypeArg[] {
        if (this._callTraceTypeArgs == null) {
            this._callTraceTypeArgs = CallTraceTypeArg.loadAll()
        }

        return this._callTraceTypeArgs!
    }

    /// All call trace args in the current context
    public get callTraceArgs(): CallTraceArg[] {
        if (this._callTraceArgs == null) {
            this._callTraceArgs = CallTraceArg.loadAll()
        }

        return this._callTraceArgs!
    }
}
