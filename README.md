# portablegabi-pallet

This pallet can be used to store an accumulator on chain.

## Installation

### Runtime `Cargo.toml`

To add this pallet to your runtime, simply include the following to your runtime's `Cargo.toml` file:

```TOML
[dependencies.portablegabi-pallet]
default-features = false
git = "https://github.com/KILTprotocol/portablegabi-pallet.git"
```

and update your runtime's `std` feature to include this pallet:

```TOML
std = [
    # --snip--
    "portablegabi-pallet/std",
]
```

### Runtime `lib.rs`

You should implement it's trait like so:

```rust
impl portablegabi_pallet::Trait for Runtime {
	type Event = Event;
}
```

and include it in your `construct_runtime!` macro:

```rust
PortablegabiPallet: portablegabi_pallet::{Module, Call, Storage, Event<T>},
```

### Genesis Configuration

This template pallet does not have any genesis configuration.

## Reference Docs

You can view the reference docs for this pallet by running:

```
cargo doc --open
```

or by visiting this site: [kilt.io/developers](https://kilt.io/developers/)
