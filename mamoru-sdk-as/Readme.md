## AssemblyScript SDK for Mamoru Daemons

This is an SDK for interacting with Mamoru from AssemblyScript.

### Installation

You can install this package via npm:

```bash
npm install @mamoru-ai/mamoru-sdk-as
```

### Usage

Here is an example Mamoru Daemon that uses the SDK:

```typescript
import { http, HttpMethod, report, IncidentSeverity, query } from '@mamoru-ai/mamoru-sdk-as/assembly';

export function main(): void {
    const response = http(
        HttpMethod.GET,
        'https://example.com',
        );

    const error = response.error();

    if (error != null) {
        // Handle the error
    } else {
        const body = response.body();
        if (body) {
            // Do something with the response body
            report(IncidentSeverity.Info, "Response body received");
        }
    }

    query('SELECT * FROM transactions').forEach((tx) => {
        const hash = tx.getString!("hash")!.valueOf();

        if (hash == "0x0") {
           report(IncidentSeverity.Info, "Hash is 0x0");
        }
    });
}
```

The SDK offers these functions:

```typescript
/**
 * Performs HTTP request to a remote host
 * 
 * @returns {HttpResponse} The response or an error
 *
 * @example
 * let response = http(HttpMethod.GET, "https://example.com/");
 * if (response.status() == 200) {
 *     //
 * }
 */
http(
    method: HttpMethod,
    url: string,
    headers: Map<string, string> | null = null,
    body: string | null = null,
): HttpResponse;

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
query(query: string): Array<JSON.Obj>;

/**
 * Reports an incident to Validation Chain.
 *
 * @example
 * let data = new IncidentDataStruct();
 *
 * data.addNull("null");
 * data.addNumber("number", 42.0);
 * data.addString("string", "hello");
 * data.addBoolean("boolean", true);
 * data.addList("list", [
 *     new StringDataValue("first"),
 *     new StringDataValue("second"),
 * ]);
 *
 * report(IncidentSeverity.Alert, "Test", data);
 */
report(
    severity: IncidentSeverity,
    message: string,
    data: IncidentDataStruct | null = null,
    address: string = "",
): void;

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
parameter(key: string): DaemonParameter;
```

### License

This SDK is licensed under the MIT License.
