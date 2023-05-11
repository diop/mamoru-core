import { _mamoru_get_call_trace_arg_by_id, _mamoru_get_call_trace_args, _mamoru_get_call_trace_type_args, _mamoru_get_call_traces } from "./imports";
import { readMemory, unpackValues } from "@mamoru-ai/mamoru-sdk-as/assembly/util";
import { Value } from "@mamoru-ai/mamoru-sdk-as/assembly";
import { Decoder } from "@wapc/as-msgpack/assembly";
import { SuiCtx } from "./index";

export class CallTrace {
    public readonly seq: u64
    public readonly txSeq: u64
    public readonly depth: u32
    public readonly callType: u8
    public readonly gasUsed: u64
    public readonly transactionModule: string | null
    public readonly func: string

    private _ctx: SuiCtx
    private _args: Value[] | null
    private _typeArgs: string[] | null

    private constructor(ctx: SuiCtx, seq: u64, txSeq: u64, depth: u32, callType: u8, gasUsed: u64, transactionModule: string | null, func: string) {
        this._ctx = ctx
        this._args = null
        this._typeArgs = null

        this.seq = seq;
        this.txSeq = txSeq;
        this.depth = depth;
        this.callType = callType;
        this.gasUsed = gasUsed;
        this.transactionModule = transactionModule;
        this.func = func;
    }

    /// All arguments of the current call trace
    public get args(): Value[] {
        if (this._args == null) {
            let args = new Array<Value>();
            for (let i = 0; i < this._ctx.callTraceArgs.length; i++) {
                const arg = this._ctx.callTraceArgs[i];

                if (arg.callTraceSeq == this.seq) {
                    args.push(arg.value);
                }
            }

            this._args = args
        }

        return this._args!
    }

    /// All type arguments of the current call trace
    public get typeArgs(): string[] {
        if (this._typeArgs == null) {
            let typeArgs = new Array<string>();
            for (let i = 0; i < this._ctx.callTraceTypeArgs.length; i++) {
                const arg = this._ctx.callTraceTypeArgs[i];

                if (arg.callTraceSeq == this.seq) {
                    typeArgs.push(arg.arg);
                }
            }

            this._typeArgs = typeArgs
        }

        return this._typeArgs!
    }

    public static loadAll(ctx: SuiCtx): CallTrace[] {
        let ptrLen = unpackValues(_mamoru_get_call_traces());

        return CallTrace.fromHost(ctx, ptrLen[0], ptrLen[1]);
    }


    private static fromHost(ctx: SuiCtx, ptr: u32, len: u32): CallTrace[] {
        const decoder = new Decoder(readMemory(ptr, len).buffer);

        let callTraces = new Array<CallTrace>();
        let size = decoder.readArraySize();

        for (let i: u32 = 0; i < size; i++) {
            // consume array size (we can't parse data otherwise)
            let _ = decoder.readArraySize();

            let seq = decoder.readUInt64();
            let txSeq = decoder.readUInt64();
            let depth = decoder.readUInt32();
            let callType = decoder.readUInt8();
            let gasUsed = decoder.readUInt64();

            let transactionModule: string | null;
            if (decoder.isNextNil()) {
                transactionModule = null;
            } else {
                transactionModule = decoder.readString();
            }

            let func = decoder.readString();

            callTraces.push(new CallTrace(ctx, seq, txSeq, depth, callType, gasUsed, transactionModule, func));
        }

        return callTraces;
    }
}

export class CallTraceTypeArg {
    public readonly seq: u64
    public readonly callTraceSeq: u64
    public readonly arg: string

    private constructor(seq: u64, call_trace_seq: u64, arg: string) {
        this.seq = seq;
        this.callTraceSeq = call_trace_seq;
        this.arg = arg;
    }

    public static loadAll(): CallTraceTypeArg[] {
        let ptr_len = unpackValues(_mamoru_get_call_trace_type_args());

        return CallTraceTypeArg.fromHost(ptr_len[0], ptr_len[1]);
    }

    private static fromHost(ptr: u32, len: u32): CallTraceTypeArg[] {
        const decoder = new Decoder(readMemory(ptr, len).buffer);

        return decoder.readArray<CallTraceTypeArg>((decoder: Decoder) => {
            // consume array size (we can't parse data otherwise)
            let _ = decoder.readArraySize();

            let seq = decoder.readUInt64();
            let call_trace_seq = decoder.readUInt64();
            let arg = decoder.readString();

            return new CallTraceTypeArg(seq, call_trace_seq, arg);
        });
    }
}

export class CallTraceArg {
    public readonly seq: u64
    public readonly callTraceSeq: u64

    private _value: Value | null = null;

    public get value(): Value {
        if (this._value == null) {
            const ptr_len = unpackValues(_mamoru_get_call_trace_arg_by_id(this.seq));
            const buffer = readMemory(ptr_len[0], ptr_len[1]).buffer;

            this._value = Value.fromBytes(buffer)
        }

        return this._value!;
    }

    private constructor(seq: u64, call_trace_seq: u64) {
        this.seq = seq;
        this.callTraceSeq = call_trace_seq;
    }

    public static loadAll(): CallTraceArg[] {
        let ptr_len = unpackValues(_mamoru_get_call_trace_args());

        return CallTraceArg.fromHost(ptr_len[0], ptr_len[1]);
    }

    private static fromHost(ptr: u32, len: u32): CallTraceArg[] {
        const decoder = new Decoder(readMemory(ptr, len).buffer);

        return decoder.readArray<CallTraceArg>((decoder: Decoder) => {
            // consume array size (we can't parse data otherwise)
            let _ = decoder.readArraySize();

            let seq = decoder.readUInt64();
            let call_trace_seq = decoder.readUInt64();

            return new CallTraceArg(seq, call_trace_seq);
        });
    }
}
