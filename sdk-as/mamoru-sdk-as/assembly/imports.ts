@external("mamoru", "http")
export declare function _mamoru_http(request: string): string

@external("mamoru", "query")
export declare function _mamoru_query(query: string): string

@external("mamoru", "parameter")
export declare function _mamoru_parameter(key: string): string

@external("mamoru", "report")
export declare function _mamoru_report(incident: string): void

@external("mamoru", "u256_from_str")
export declare function _mamoru_u256_from_str(payload: string): u64

@external("env", "assert")
export declare function _env_assert(condition: i32, message: string): void
