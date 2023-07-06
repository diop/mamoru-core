import { hexToU256 } from "../util";

describe("hexToU256", () => {
    test("from 0x0", () => {
        const number = hexToU256("0x0");

        expect(number.toString()).toBe("0");
    });

    test("from 0xA", () => {
        const number = hexToU256("0xA");

        expect(number.toString()).toBe("10");
    });

    test("Max u256", () => {
        const number = hexToU256("0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF");

        expect(number.toString(16)).toBe("ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff");
    });
});
