/*! \file */
/*******************************************
 *                                         *
 *  File auto-generated by `::safer_ffi`.  *
 *                                         *
 *  Do not manually edit this file.        *
 *                                         *
 *******************************************/

#ifndef __RUST_QUERY_VALIDATOR_C__
#define __RUST_QUERY_VALIDATOR_C__

#ifdef __cplusplus
extern "C" {
#endif


#include <stddef.h>
#include <stdint.h>

/** \remark Has the same ABI as `uint8_t` **/
#ifdef DOXYGEN
typedef enum FfiChainType
#else
typedef uint8_t FfiChainType_t; enum
#endif
{
    /** . */
    FFI_CHAIN_TYPE_SUI = 0,
    /** . */
    FFI_CHAIN_TYPE_EVM = 1,
    /** . */
    FFI_CHAIN_TYPE_APTOS = 2,
}
#ifdef DOXYGEN
FfiChainType_t
#endif
;

typedef struct FfiDaemonParameters FfiDaemonParameters_t;


#include <stdbool.h>

typedef struct FfiValidationResult {

    bool is_error;

    char * message;

} FfiValidationResult_t;

/** \brief
 *  Drops `parameters` argument.
 */
FfiValidationResult_t ffi_validate_sql (
    FfiChainType_t chain,
    char const * query,
    FfiDaemonParameters_t * parameters);

/** \brief
 *  Drops `parameters` argument.
 */
FfiValidationResult_t ffi_validate_sql_renders (
    char const * query,
    FfiDaemonParameters_t * parameters);

/** \brief
 *  `&'lt [T]` but with a guaranteed `#[repr(C)]` layout.
 * 
 *  # C layout (for some given type T)
 * 
 *  ```c
 *  typedef struct {
 *      // Cannot be NULL
 *      T * ptr;
 *      size_t len;
 *  } slice_T;
 *  ```
 * 
 *  # Nullable pointer?
 * 
 *  If you want to support the above typedef, but where the `ptr` field is
 *  allowed to be `NULL` (with the contents of `len` then being undefined)
 *  use the `Option< slice_ptr<_> >` type.
 */
typedef struct slice_ref_uint8 {

    uint8_t const * ptr;

    size_t len;

} slice_ref_uint8_t;

FfiValidationResult_t ffi_validate_assembly_script (
    FfiChainType_t chain,
    slice_ref_uint8_t bytes);

void ffi_drop_validation_result (
    FfiValidationResult_t result);

FfiDaemonParameters_t * ffi_new_daemon_parameters (void);

void ffi_append_daemon_parameter (
    FfiDaemonParameters_t * parameters,
    char const * key,
    char const * value);


#ifdef __cplusplus
} /* extern "C" */
#endif

#endif /* __RUST_QUERY_VALIDATOR_C__ */
