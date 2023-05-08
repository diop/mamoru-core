@external("mamoru_sui", "get_transactions")
export declare function _mamoru_get_transactions(): u64

@external("mamoru_sui", "get_call_traces")
export declare function _mamoru_get_call_traces(): u64

@external("mamoru_sui", "get_call_trace_type_args")
export declare function _mamoru_get_call_trace_type_args(): u64

@external("mamoru_sui", "get_call_trace_args")
export declare function _mamoru_get_call_trace_args(): u64

@external("mamoru_sui", "get_events")
export declare function _mamoru_get_events(): u64

@external("mamoru_sui", "get_call_trace_arg_by_id")
export declare function _mamoru_get_call_trace_arg_by_id(id: u64): u64
