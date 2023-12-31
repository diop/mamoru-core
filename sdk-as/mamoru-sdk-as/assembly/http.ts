import { JSON, JSONEncoder } from "assemblyscript-json/assembly";

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

export class HttpResponse {
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
        let headerKeys = headersObj.keys();
        let headers = new Map<String, String>();

        for (let i = 0; i < headerKeys.length; ++i) {
            let key = headerKeys[i];
            let value = headersObj.get(key) as JSON.Str;

            headers.set(key, value.valueOf());
        }

        return headers;
    }

    public body(): Uint8Array | null {
        let body = this.payload.getArr("body");

        if (body == null) {
            return null;
        } else {
            let bodyValue = body.valueOf();
            let array = new Uint8Array(bodyValue.length);

            for (let i = 0; i < array.length; ++i) {
                let int = bodyValue[i] as JSON.Integer;

                array[i] = int.valueOf() as u32;
            }

            return array
        }
    }
}
