#pragma once

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * This header file outlines the API between an IXI and an `iota.rs` full node.
 *
 * _Threading_:
 * All methods exposed by an IXI host via `ixi_host_t` are synchronous and
 * thread-safe.
 * All methods exposed by an IXI guest via `ixi_guest_t` are expected to be
 * thread-safe.
 *
 * _Memory_:
 * Objects are only valid for the duration of the method call for both the
 * guest and host interfaces. If a value is to be persisted, the receiving party
 * needs to memcpy it.
 *
 * _Crypto_:
 * The IXI interface does not provide a standardised way to access to the
 * various cryptographic
 * functions an IXI might use at the moment. The IXI is expected to provide its
 * own implementations.
 */

// TODO: define event structs
// TODO: define transaction/bundle models

#define IXI_API_VERSION 0x00000100 // = 0.1.0

/// Generic struct that holds an array of chars (*not null-terminated*!) or
/// trits depending only
/// what the IXI guest specified on setup.
typedef struct {
  void *data;
  size_t length;
} ixi_trinary_t;

/// Used for specifying the trinary encoding the IXI expects.
typedef enum {
  /// `data` element in `ixi_trinary_t` will hold chars
  IXI_TRINARY_CHAR = 0,
  /// `data` element in `ixi_trinary_t` will hold uint8_t
  IXI_TRINARY_TRIT,
} ixi_trinary_type_t;

/// Generic IXI version identifier.
/// Expected to be of the format 0xMMMmmmpp where
/// MMM = major version, mmm = minor version, pp = patch level
typedef int32_t ixi_version_t;

typedef enum {
  IXI_TRANSACTION = 0,
  IXI_BUNDLE,
  IXI_ADDRESS,
} ixi_type;

typedef enum {
  IXI_EVT_TRANSACTION = 1,
  IXI_EVT_BALANCE_CHANGE,
} ixi_event_t;

typedef enum { IXI_SUCCESS = 0, IXI_ERR_INVALID_ARGUMENT = 500 } ixi_result_t;

typedef struct {
  ixi_result_t status;
  /// Unique identifier for the subscription
  uint32_t subscription_id;
} ixi_subscribe_to_s;

typedef struct {
  ixi_result_t status;
  /// Value implementation depends on ixi_type parameter passed to the lookup
  /// method.
  void *result;
} ixi_lookup_s;

/// Encodes lookup results for ADDRESS
typedef struct {
  ixi_trinary_t address;
  uint32_t balance;
} ixi_lookup_address_s;

/// Callback method signature for event subscriptions
typedef void (*ixi_event_callback_t)(ixi_event_t, void *);

typedef struct {
  /// Subscribe to events of a specific type
  ixi_subscribe_to_s (*subscribe)(ixi_event_t, ixi_event_callback_t, void *);
  /// Cancel an event subscription
  ixi_result_t (*unsubscribe)(uint32_t);
  /// Attach the provided trytes to the tangle and performs PoW
  ixi_result_t (*attach_to_tangle)(ixi_trinary_t);
  /// Looks up a transaction/bundle/address
  ixi_lookup_s (*lookup)(ixi_type, void *);
} ixi_host_s;

typedef struct {
  /// The encoding of `ixi_trinary_t` that the host uses
  ixi_trinary_type_t encoding;
} ixi_guest_s;

/// Called by the host on loading the IXI.
/// The guest must initialise and return a guest structure for use by the host.
ixi_guest_s ixi_on_load(ixi_host_s, ixi_version_t);

/// Called by the IXI host on unloading the IXI.
void ixi_on_unload(void);

#ifdef __cplusplus
}
#endif
