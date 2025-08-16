# azathoth_utils
![Crates.io Version](https://img.shields.io/crates/v/azathoth-utils)

A collection of `no-std` utilities used by the [AzathothC2 framework](https://github.com/AzathothC2/)
Modules are feature-gated for minimal binary size and can be enabled as needed.

>[!WARNING]
> **Be advised that this is still a WIP crate and may change at any time! (Unstable)**


## Features

* `hasher` – Identifier/symbol hashing helpers for obfuscated lookups.
* `formatter` – Lightweight formatting helpers for constrained environments where alloc formatters may fail or be unsafe.
* `psearch` – Extendable pattern search utilities with optional wildcard support.
* `codec` – Minimal data encoding/decoding helpers.
* `errors` – Common error types and aliases used across modules (always enabled).

## Installation

Add the crate via Cargo:
```cargo add azathoth_utils```

Or manually in `Cargo.toml`: ```azathoth_utils = "0.1.1";```

Enable optional features as needed:
```
azathoth_utils = { version = "0.1.1", features = ["hasher", "psearch"] }
```

## Example: CRC32 checksum

```rust
use azathoth_utils::crc32;

let checksum = crc32(b"deadbeef");
assert_eq!(checksum, 0x52_8f_6f_ca);
```

## License
MIT
