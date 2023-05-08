import { Value } from "../value";


/*

The bytes are build from the following Rust code:

    let mut fields = HashMap::new();
    fields.insert("b".to_string(), Value::U64(2));
    fields.insert("c".to_string(), Value::String("test".to_string()));
    fields.insert(
        "d".to_string(),
        Value::List(vec![Value::Bool(true), Value::Bool(false)]),
    );

    let struct_value = StructValue::new("test".to_string(), fields);
    let value = Value::Struct(struct_value);
    let value_data = ValueData::new(value).unwrap();
    let bytes = value_data.as_ref();
    dbg!(&bytes);
 */
const VALUE_EXAMPLE = [
    129,
    162,
    115,
    116,
    146,
    164,
    116,
    101,
    115,
    116,
    131,
    161,
    99,
    129,
    161,
    115,
    164,
    116,
    101,
    115,
    116,
    161,
    98,
    129,
    163,
    117,
    54,
    52,
    2,
    161,
    100,
    129,
    161,
    108,
    146,
    129,
    161,
    98,
    195,
    129,
    161,
    98,
    194,
];

describe("Value Smoke", () => {
    test("from predefined bytes", () => {
        let value_example = new Uint8Array(VALUE_EXAMPLE.length);
        value_example.set(VALUE_EXAMPLE);

        const value = Value.fromBytes(value_example.buffer)

        const struct = value.asStruct()!;
        expect(struct.type).toBe("test");

        const field_b = struct.fields.get("b");
        expect(field_b.asU64()!.value).toBe(2);

        const field_c = struct.fields.get("c");
        expect(field_c.asString()!.value).toBe("test");

        const field_d = struct.fields.get("d");
        const b_values = field_d.asList()!.values;
        expect(b_values[0].asBool()!.value).toBe(true);
        expect(b_values[1].asBool()!.value).toBe(false);
    });
});
