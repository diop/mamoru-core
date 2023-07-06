@external("mamoru_evm", "get_blocks")
export declare function _mamoru_get_blocks(): u64

@external("mamoru_evm", "get_transactions")
export declare function _mamoru_get_transactions(): u64

@external("mamoru_evm", "get_call_traces")
export declare function _mamoru_get_call_traces(): u64

@external("mamoru_evm", "get_events")
export declare function _mamoru_get_events(): u64

@external("mamoru_evm", "parse_tx_input")
export declare function _mamoru_parse_tx_input(abi: string, data: string): i64
