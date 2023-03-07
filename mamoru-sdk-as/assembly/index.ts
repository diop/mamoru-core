// The entry file of your WebAssembly module.

import { JSON, JSONEncoder } from "assemblyscript-json/assembly";

import {
    _mamoru_query,
    _mamoru_report,
    _mamoru_http,
    _mamoru_parameter,
} from "./imports";

// Performs HTTP request to a remote host
// Returns an HTTP response or an error in `HttpResponse`.
export function http(request: HttpRequest): HttpResponse {
    let payload: string = _mamoru_http(request.toJSON());

    return HttpResponse.fromJSON(payload)
}

// Queries the daemon context.
// `query` parameter must be a valid SQL query for the network daemon running in.
// Returns a list of objects, where object keys are the fields returned by the query.
export function query(query: string): Array<JSON.Obj> {
    let query_result: string = _mamoru_query(query);
    let json: JSON.Arr = <JSON.Arr>(JSON.parse(query_result));

    return json.valueOf().map((value: JSON.Value) => value as JSON.Obj);
}

// Reports an incident.
export function report(): void {
    _mamoru_report()
}

// Returns a `key` parameter defined for the daemon.
// The return value is `DaemonParameter` object from which
// you can receive a specific type:
// ```
// let param = parameter("foo");
// let maybe_bool = param.asBoolean();
// let maybe_number = param.asNumber();
// let str = param.asString();
// ```
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

export enum HttpMethod {
    POST,
    GET,
    PUT,
    PATCH,
    DELETE,
}

function httpMethodToString(method: HttpMethod): string {
    switch (method) {
        case HttpMethod.POST:
            return "POST"
        case HttpMethod.GET:
            return "GET"
        case HttpMethod.PUT:
            return "PUT"
        case HttpMethod.PATCH:
            return "PATCH"
        case HttpMethod.DELETE:
            return "DELETE"
        default:
            return "UNDEFINED"
    }
}

export class HttpRequest {
    method: HttpMethod
    url: string
    body: string | null
    headers: Map<string, string>

    public constructor(
        method: HttpMethod,
        url: string,
        headers: Map<string, string> | null = null,
        body: string | null = null,
    ) {
        this.method = method;
        this.url = url;
        this.body = body;

        if (headers != null) {
            this.headers = headers;
        } else {
            this.headers = new Map<string, string>();
        }
    }

    toJSON(): string {
        let encoder = new JSONEncoder();
        encoder.pushObject(null);
        encoder.setString("method", httpMethodToString(this.method));
        encoder.setString("url", this.url);

        let body = this.body;
        if (body != null) {
            encoder.setString("body", body);
        }

        {
            encoder.pushObject("headers");
            let keys = this.headers.keys();

            for (let i = 0; i < keys.length; ++i) {
                let key = keys[i];

                encoder.setString(key, this.headers.get(key));
            }

            encoder.popObject();
        }

        encoder.popObject();

        return encoder.toString();
    }
}

class HttpResponse {
    payload: JSON.Obj

    constructor(payload: JSON.Obj) {
        this.payload = payload
    }

    static fromJSON(json: string): HttpResponse {
        let parsed: JSON.Obj = <JSON.Obj>(JSON.parse(json));

        return new HttpResponse(parsed)
    }

    public status(): u16 {
        // this cast should be safe, as `u16` is used on the host system
        return this.payload.getInteger("status")!.valueOf() as u16
    }

    public error(): string | null {
        let error = this.payload.getString("error");

        if (error == null) {
            return null;
        } else {
            return error.valueOf();
        }
    }

    public headers(): Map<String, String> {
        let headersObj = this.payload.getObj("headers")!.valueOf();
        let headers = new Map<String, String>();

        headersObj.keys().forEach((key) => {
            let value = headersObj.get(key) as JSON.Str;
            headers.set(key, value.valueOf());
        });

        return headers;
    }

    public body(): Uint8Array | null {
        let body = this.payload.getArr("body");

        if (body == null) {
            return null;
        } else {
            let bodyValue = body.valueOf();
            let array = new Uint8Array(bodyValue.length);

            bodyValue.forEach((val, idx) => {
                let int = val as JSON.Integer;

                array[idx] = int.valueOf();
            });

            return array
        }
    }
}
