# azathoth_utils

A collection of `no-std` utilities used by the [AzathothC2 framework](https://github.com/AzathothC2/)
Modules are feature-gated for minimal binary size and can be enabled as needed.

## Features

* `hasher` – Identifier/symbol hashing helpers for obfuscated lookups.
* `formatter` – Lightweight formatting helpers for constrained environments where alloc formatters may fail or be unsafe.
* `psearch` – Extendable pattern search utilities with optional wildcard support.
* `codec` – Minimal data encoding/decoding helpers.
* `errors` – Common error types and aliases used across modules (always enabled).

## Installation

Add the crate via Cargo:
```cargo add azathoth_utils```

Or manually in `Cargo.toml`: ```azathoth_utils = "0.1.0";```

Enable optional features as needed:
```
azathoth_utils = { version = "0.1.0", features = ["hasher", "psearch"] }
```

## Example: CRC32 checksum

```rust
use azathoth_utils::crc32;

let checksum = crc32(b"deadbeef");
assert_eq!(checksum, 0x52_8f_6f_ca);
```

>![WARNING]
> **Be advised that this is still a WIP crate and may change at any time! (Unstable)**


## Feature examples
Below are examples for how to use each feature in this crate

### `hasher`
The `Hasher` trait  
```rust
use azathoth_utils::hasher::Hasher;

let crc32_name_hasher = |name: &str| -> u32 {
    azathoth_utils::crc32(name.as_bytes())
};

let hval = crc32_name_hasher.hash("LoadLibraryA");

let salted = (|name: &str, salt: u32| -> u32 {
    azathoth_utils::crc32(name.as_bytes()) ^ salt
}, 0xDEAD_BEEF);

let salted_val = salted.hash("GetProcAddress");

let bytes_val = crc32_name_hasher.hash_bytes(&[0xFF, 0xFE, 0xFD]);
```


## License
MIT