import { Decoder } from "@wapc/as-msgpack/assembly";
import { i256Safe, u256 } from "@mamoru-ai/mamoru-sdk-as/assembly";
import { hexToU256, msgPackReadUint8Array } from "@mamoru-ai/mamoru-sdk-as/assembly/util";

const BOOL_VALUE_TAG: string = "Bool";
const UINT_VALUE_TAG: string = "Uint";
const INT_VALUE_TAG: string = "Int";
const STRING_VALUE_TAG: string = "String";
const ADDRESS_VALUE_TAG: string = "Address";
const BYTES_VALUE_TAG: string = "Bytes";
const FIXED_BYTES_VALUE_TAG: string = "FixedBytes";
const ARRAY_VALUE_TAG: string = "Array";
const FIXED_ARRAY_VALUE_TAG: string = "FixedArray";
const TUPLE_VALUE_TAG: string = "Tuple";

export class Value {
    private readonly type: string

    protected constructor(type: string) {
        this.type = type;
    }

    public static fromBytes(data: ArrayBuffer): Value[] {
        let decoder = new Decoder(data);

        return Value.fromDecoder(decoder);
    }


    private static fromDecoder(decoder: Decoder): Value[] {
        return decoder.readArray<Value>((decoder: Decoder) => {
            const _ = decoder.readMapSize();
            const type = decoder.readString();

            if (type == BOOL_VALUE_TAG) {
                return new BoolValue(decoder.readBool());
            }

            if (type == UINT_VALUE_TAG) {
                const number: u256 = hexToU256(decoder.readString());

                return new UintValue(number);
            }

            if (type == INT_VALUE_TAG) {
                const number: u256 = hexToU256(decoder.readString());

                // i256 doesn't support fromBytes yet
                // so, I pass through u256
                return new IntValue(new i256Safe(
                    number.lo1,
                    number.lo2,
                    number.hi1,
                    number.hi2,
                ));
            }

            if (type == STRING_VALUE_TAG) {
                return new StringValue(decoder.readString());
            }

            if (type == ADDRESS_VALUE_TAG) {
                const hex = decoder.readString();
                return new AddressValue(hex);
            }

            if (type == BYTES_VALUE_TAG) {
                return new BytesValue(msgPackReadUint8Array(decoder));
            }

            if (type == FIXED_BYTES_VALUE_TAG) {
                return new FixedBytesValue(msgPackReadUint8Array(decoder));
            }

            if (type == ARRAY_VALUE_TAG) {
                return new ArrayValue(Value.fromDecoder(decoder));
            }

            if (type == FIXED_ARRAY_VALUE_TAG) {
                return new FixedArrayValue(Value.fromDecoder(decoder));
            }

            if (type == TUPLE_VALUE_TAG) {
                return new TupleValue(Value.fromDecoder(decoder));
            }

            throw new Error("unknown `Value` type: " + type);
        })
    }

    public isBool(): bool {
        return this.type == BOOL_VALUE_TAG;
    }

    public asBool(): bool {
        return (this as BoolValue).value;
    }

    public isUint(): bool {
        return this.type == UINT_VALUE_TAG;
    }

    public asUint(): u256 {
        return (this as UintValue).value;
    }

    public isInt(): bool {
        return this.type == INT_VALUE_TAG;
    }

    public asInt(): i256Safe {
        return (this as IntValue).value;
    }

    public isString(): bool {
        return this.type == STRING_VALUE_TAG;
    }

    public asString(): string {
        return (this as StringValue).value;
    }

    public isAddress(): bool {
        return this.type == ADDRESS_VALUE_TAG;
    }

    public asAddress(): string {
        return (this as AddressValue).value;
    }

    public isBytes(): bool {
        return this.type == BYTES_VALUE_TAG;
    }

    public asBytes(): Uint8Array {
        return (this as BytesValue).value;
    }

    public isFixedBytes(): bool {
        return this.type == FIXED_BYTES_VALUE_TAG;
    }

    public asFixedBytes(): Uint8Array {
        return (this as FixedBytesValue).value;
    }

    public isArray(): bool {
        return this.type == ARRAY_VALUE_TAG;
    }

    public asArray(): Value[] {
        return (this as ArrayValue).value;
    }

    public isFixedArray(): bool {
        return this.type == FIXED_ARRAY_VALUE_TAG;
    }

    public asFixedArray(): Value[] {
        return (this as FixedArrayValue).value;
    }

    public isTuple(): bool {
        return this.type == TUPLE_VALUE_TAG;
    }

    public asTuple(): Value[] {
        return (this as TupleValue).value;
    }
}

class BoolValue extends Value {
    public readonly value: bool

    constructor(value: bool) {
        super(BOOL_VALUE_TAG);
        this.value = value;
    }
}

class UintValue extends Value {
    public readonly value: u256

    constructor(value: u256) {
        super(UINT_VALUE_TAG);
        this.value = value;
    }
}

class IntValue extends Value {
    public readonly value: i256Safe

    constructor(value: i256Safe) {
        super(INT_VALUE_TAG);
        this.value = value;
    }
}

class StringValue extends Value {
    public readonly value: string

    constructor(value: string) {
        super(STRING_VALUE_TAG);
        this.value = value;
    }
}

class AddressValue extends Value {
    public readonly value: string

    constructor(value: string) {
        super(ADDRESS_VALUE_TAG);
        this.value = value;
    }
}

class BytesValue extends Value {
    public readonly value: Uint8Array

    constructor(value: Uint8Array) {
        super(BYTES_VALUE_TAG);
        this.value = value;
    }
}

class FixedBytesValue extends Value {
    public readonly value: Uint8Array

    constructor(value: Uint8Array) {
        super(FIXED_BYTES_VALUE_TAG);
        this.value = value;
    }
}

class ArrayValue extends Value {
    public readonly value: Value[]

    constructor(value: Value[]) {
        super(ARRAY_VALUE_TAG);
        this.value = value;
    }
}

class FixedArrayValue extends Value {
    public readonly value: Value[]

    constructor(value: Value[]) {
        super(FIXED_ARRAY_VALUE_TAG);
        this.value = value;
    }
}

class TupleValue extends Value {
    public readonly value: Value[]

    constructor(value: Value[]) {
        super(TUPLE_VALUE_TAG);
        this.value = value;
    }
}
