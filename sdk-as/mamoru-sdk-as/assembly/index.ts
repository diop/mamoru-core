// The entry file of your WebAssembly module.
import { JSON } from "assemblyscript-json/assembly";

import { _env_assert, _mamoru_http, _mamoru_parameter, _mamoru_query, _mamoru_report, } from "./imports";

import { HttpMethod, HttpRequest, HttpResponse } from "./http";
import { Incident, IncidentSeverity, } from "./incident";
import { Value } from "./value";
import { i128, i256Safe, u128, u256 } from "as-bignum/assembly";

export * from "./util";

export {
    i128,
    u128,
    u256,
    i256Safe,
    JSON,
    HttpMethod,
    HttpRequest,
    HttpResponse,
    IncidentSeverity,
    Value,
};

/**
 * Performs HTTP request to a remote host
 *
 * @returns {HttpResponse} The response or an error
 *
 * @example
 * let response = http(HttpMethod.GET, "https://example.com/");
 * if (response.status() == 200) {
 *     // handle response
 * }
 */
export function http(
    method: HttpMethod,
    url: string,
    headers: Map<string, string> | null = null,
    body: string | null = null,
): HttpResponse {
    const request = new HttpRequest(method, url, headers, body);
    let payload: string = _mamoru_http(request.toJSON());

    return HttpResponse.fromJSON(payload)
}

/**
 * Queries the daemon context.
 * @param query Valid SQL query for the network.
 *
 * @return {Array<JSON.Obj>} Object keys are the fields returned by the query.
 *
 * @example
 * let rows = query("SELECT t.gas_used FROM transactions t WHERE t.digest = 'DUMMY_HASH'");
 *
 * rows.forEach(value => {
 *     let gas_used = value.getInteger("gas_used")!.valueOf();
 *
 *     if (gas_used == 42_000) {
 *         // handle value
 *     }
 * });
 */
export function query(query: string): JSON.Obj[] {
    let query_result: string = _mamoru_query(query);
    let json: JSON.Arr = <JSON.Arr>(JSON.parse(query_result));

    return json.valueOf().map((value: JSON.Value) => value as JSON.Obj);
}

/**
 * Reports an incident to Validation Chain.
 *
 * @example
 *
 * report(tx.txHash, IncidentSeverity.Alert, "Test");
 */
export function report(
    txHash: string,
    severity: IncidentSeverity,
    message: string,
    data: Uint8Array | null = null,
    address: string = "",
): void {
    _mamoru_report((new Incident(txHash, severity, message, data, address)).toJSON())
}

/**
 * Retrieve a parameter value from the daemon configuration.
 *
 * @returns {DaemonParameter}
 *
 * @example
 * let param = parameter("foo");
 * let maybe_bool = param.asBoolean();
 * let maybe_number = param.asNumber();
 * let str = param.asString();
 */
export function parameter(key: string): DaemonParameter {
    let parameter: string = _mamoru_parameter(key);

    return new DaemonParameter(parameter)
}

class DaemonParameter {
    value: string

    public constructor(value: string) {
        this.value = value;
    }

    public asString(): string {
        return this.value
    }

    public asBoolean(): boolean {
        return this.value == 'true'
    }

    public asNumber(): number {
        return parseFloat(this.value);
    }
}

/**
 * Test-only function to assert a condition.
 * @param condition
 * @param message
 */
export function assert(condition: boolean, message: string | null): void {
    if (message == null) {
        message = "Assertion failed";
    }

    _env_assert(condition ? 1 : 0, message)
}
