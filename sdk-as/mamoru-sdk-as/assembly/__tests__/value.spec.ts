import { Value } from "../value";
import { u128 } from "../";

/**
 * Run `cargo test -p mamoru-core value::tests::test_value_serialize -- --ignored --nocapture`
 * to generate this value
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
    132,
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
    101,
    129,
    161,
    115,
    167,
    48,
    120,
    49,
    97,
    100,
    98,
    98,
];

describe("Value Smoke", () => {
    test("from predefined bytes", () => {
        let value_example = new Uint8Array(VALUE_EXAMPLE.length);
        value_example.set(VALUE_EXAMPLE);

        const value = Value.fromBytes(value_example.buffer)

        expect(value.isStruct()).toBe(true);
        const struct = value.asStruct();
        expect(struct.type).toBe("test");

        const field_b = struct.fields.get("b");
        expect(field_b.asU64()).toBe(2);

        const field_c = struct.fields.get("c");
        expect(field_c.asString()).toBe("test");

        const field_d = struct.fields.get("d");
        const b_values = field_d.asList();
        expect(b_values[0].asBool()).toBe(true);
        expect(b_values[1].asBool()).toBe(false);

        const field_e = struct.fields.get("e");
        expect(field_e.asU128()).toBe(u128.from(110011));
    });
});
