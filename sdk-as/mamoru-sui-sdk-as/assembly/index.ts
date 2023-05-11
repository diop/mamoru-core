import { Transaction } from "./tx";
import { Event } from "./event";
import { CallTrace, CallTraceArg, CallTraceTypeArg } from "./call_trace";

export {
    Transaction,
    Event,
    CallTrace,
    CallTraceArg,
    CallTraceTypeArg,
}

export class SuiCtx {
    private _tx: Transaction | null
    private _events: Event[] | null
    private _callTraces: CallTrace[] | null
    private _callTraceTypeArgs: CallTraceTypeArg[] | null
    private _callTraceArgs: CallTraceArg[] | null

    private constructor() {
        this._tx = null
        this._events = null
        this._callTraces = null
        this._callTraceTypeArgs = null
        this._callTraceArgs = null
    }

    public static load(): SuiCtx {
        return new SuiCtx()
    }

    /// The transaction of the current context
    public get tx(): Transaction {
        if (this._tx == null) {
            const txs = Transaction.loadAll();

            this._tx = txs[0]
        }

        return this._tx!
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
