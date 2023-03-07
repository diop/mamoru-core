## AssemblyScript SDK for Mamoru Daemons

This is an SDK for interacting with Mamoru from AssemblyScript.
The SDK offers these functions:

```
// Performs HTTP request to a remote host
// Returns an HTTP response or an error in `HttpResponse`.
http(request: HttpRequest): HttpResponse;

// Queries the daemon context.
// `query` parameter must be a valid SQL query for the network daemon running in.
// Returns a list of objects, where object keys are the fields returned by the query.
query(query: string): Array<JSON.Obj>;

// Reports an incident.
report(): void;

// Returns a `key` parameter defined for the daemon.
// The return value is `DaemonParameter` object from which 
// you can receive a specific type:
// ```
// let param = parameter("foo");
// let maybe_bool = param.asBoolean();
// let maybe_number = param.asNumber();
// let str = param.asString();
// ```
parameter(key: string): DaemonParameter;
```

### Installation

You can install this package via npm:

```bash
npm install @mamoru-ai/mamoru-sdk-as
```

### Usage

Here is an example Mamoru Daemon that uses the SDK:

```typescript
import { http, HttpMethod, HttpRequest, report, query } from '@mamoru-ai/mamoru-sdk-as/assembly';

export function main(): void {
    const response = http(new HttpRequest(HttpMethod.GET, 'https://example.com'));

    const error = response.error();

    if (error != null) {
        // Handle the error
    } else {
        const body = response.body();
        if (body) {
            // Do something with the response body
            report()
        }
    }

    query('SELECT * FROM transactions').forEach((tx) => {
        if (tx.hash == "0x0") {
            report();
        }
    });
}

```

### License

This SDK is licensed under the MIT License.
