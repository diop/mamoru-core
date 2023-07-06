import { decode } from "as-base64/assembly"
import { Value } from "../value";

/**
 * Run `cargo test -p mamoru-core evm_value::tests::generate_test_tx_input -- --ignored --nocapture`
 * to generate the test value.
 */
const TEST_VALUES: string = "moGnQWRkcmVzc9kqMHgwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwgapGaXhlZEJ5dGVzkwECA4GlQnl0ZXOTAQIDgaNJbnSjMHgxgaRVaW50ozB4MYGkQm9vbMOBplN0cmluZ6R0ZXN0gapGaXhlZEFycmF5koGkQm9vbMOBpEJvb2zCgaVBcnJheZKBpEJvb2zDgaRCb29swoGlVHVwbGWSgaRCb29sw4GkQm9vbMI=";

function loadTestValues(): Value[] {
    const bytes = decode(TEST_VALUES);

    return Value.fromBytes(bytes.buffer);
}

describe("Value", () => {
    test("smoke", () => {
        const values = loadTestValues();

        expect(values.length).toBe(10);

        const address = values[0].asAddress();
        expect(address).toBe("0x0000000000000000000000000000000000000000");

        const fixedBytes = values[1].asFixedBytes();
        let expectedFixedBytes = new Uint8Array(3);
        expectedFixedBytes.set([1, 2, 3]);
        expect(fixedBytes.toString()).toBe(expectedFixedBytes.toString());

        const bytes = values[2].asBytes();
        let expectedBytes = new Uint8Array(3);
        expectedBytes.set([1, 2, 3]);
        expect(bytes.toString()).toBe(expectedBytes.toString());

        const int = values[3].asInt();
        expect(int.lo1).toBe(1);

        const uint = values[4].asUint();
        expect(uint.lo1).toBe(1);

        const bool = values[5].asBool();
        expect(bool).toBe(true);

        const string = values[6].asString();
        expect(string).toBe("test");

        const fixedArray = values[7].asFixedArray();
        expect(fixedArray.length).toBe(2);
        expect(fixedArray[0].asBool()).toBe(true);
        expect(fixedArray[1].asBool()).toBe(false);

        const array = values[8].asArray();
        expect(array.length).toBe(2);
        expect(array[0].asBool()).toBe(true);
        expect(array[1].asBool()).toBe(false);

        const tuple = values[9].asTuple();
        expect(tuple.length).toBe(2);
        expect(tuple[0].asBool()).toBe(true);
        expect(tuple[1].asBool()).toBe(false);
    })
});
