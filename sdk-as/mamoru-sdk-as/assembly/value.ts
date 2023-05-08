import { Decoder } from "@wapc/as-msgpack/assembly";

const BOOL_VALUE_TAG: string = "b";
const U64_VALUE_TAG: string = "u64";
const STRING_VALUE_TAG: string = "s";
const LIST_VALUE_TAG: string = "l";
const STRUCT_VALUE_TAG: string = "st";

export class Value {
    private readonly _type: string;

    protected constructor(type: string) {
        this._type = type;
    }

    public static fromBytes(data: ArrayBuffer): Value {
        let decoder = new Decoder(data);

        return Value.fromDecoder(decoder);
    }

    private static fromDecoder(decoder: Decoder): Value {
        const _ = decoder.readMapSize();
        const type = decoder.readString();

        if (type == BOOL_VALUE_TAG) {
            return new BoolValue(decoder.readBool());
        }

        if (type == U64_VALUE_TAG) {
            return new U64Value(decoder.readUInt64());
        }

        if (type == STRING_VALUE_TAG) {
            return new StringValue(decoder.readString());
        }

        if (type == LIST_VALUE_TAG) {
            return new ListValue(decoder.readArray<Value>((decoder: Decoder) => {
                return Value.fromDecoder(decoder);
            }));
        }

        if (type == STRUCT_VALUE_TAG) {
            const _ = decoder.readArraySize();
            const ty = decoder.readString();

            return new StructValue(ty, decoder.readMap<string, Value>((decoder: Decoder) => {
                return decoder.readString();
            }, (decoder: Decoder) => {
                return Value.fromDecoder(decoder);
            }));
        }

        throw new Error("unknown `Value` type: " + type);
    }

    public asBool(): BoolValue | null {
        if (this._type == BOOL_VALUE_TAG) {
            return this as BoolValue;
        }

        return null;
    }

    public asU64(): U64Value | null {
        if (this._type == U64_VALUE_TAG) {
            return this as U64Value;
        }

        return null;
    }

    public asString(): StringValue | null {
        if (this._type == STRING_VALUE_TAG) {
            return this as StringValue;
        }

        return null;
    }

    public asList(): ListValue | null {
        if (this._type == LIST_VALUE_TAG) {
            return this as ListValue;
        }

        return null;
    }

    public asStruct(): StructValue | null {
        if (this._type == STRUCT_VALUE_TAG) {
            return this as StructValue;
        }

        return null;
    }
}

export class BoolValue extends Value {
    private readonly _value: bool;

    public constructor(value: bool) {
        super(BOOL_VALUE_TAG);

        this._value = value;
    }

    public get value(): bool {
        return this._value;
    }
}

export class U64Value extends Value {
    private readonly _value: u64;

    public constructor(value: u64) {
        super(U64_VALUE_TAG);

        this._value = value;
    }

    public get value(): u64 {
        return this._value;
    }
}

export class StringValue extends Value {
    private readonly _value: string;

    public constructor(value: string) {
        super(STRING_VALUE_TAG);

        this._value = value;
    }

    public get value(): string {
        return this._value;
    }
}

export class ListValue extends Value {
    private readonly _values: Value[];

    public constructor(value: Value[]) {
        super(LIST_VALUE_TAG);

        this._values = value;
    }

    public get values(): Value[] {
        return this._values;
    }
}

export class StructValue extends Value {
    private readonly _ty: string;
    private readonly _value: Map<string, Value>;

    public constructor(ty: string, value: Map<string, Value>) {
        super(STRUCT_VALUE_TAG);

        this._ty = ty;
        this._value = value;
    }

    public get type(): string {
        return this._ty;
    }

    public get fields(): Map<string, Value> {
        return this._value;
    }
}
