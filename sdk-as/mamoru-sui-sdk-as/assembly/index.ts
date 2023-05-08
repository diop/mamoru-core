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

    public get tx(): Transaction {
        if (this._tx == null) {
            const txs = Transaction.loadAll();

            this._tx = txs[0]
        }

        return this._tx!
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

    public get callTraceTypeArgs(): CallTraceTypeArg[] {
        if (this._callTraceTypeArgs == null) {
            this._callTraceTypeArgs = CallTraceTypeArg.loadAll()
        }

        return this._callTraceTypeArgs!
    }

    public get callTraceArgs(): CallTraceArg[] {
        if (this._callTraceArgs == null) {
            this._callTraceArgs = CallTraceArg.loadAll()
        }

        return this._callTraceArgs!
    }
}
