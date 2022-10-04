# Uke Protocol Pallet

The Uke pallet implements a basic username and message storage system that adheres to the Uke Protocol.

## Overview

The Uke pallet provides the functionality to perform basic messaging and identity assignment to
users on a given Substrate chain.

- Allows for the propagation and retrieval of messages through a given Substrate instance.

- Allows for a username to be mapped / assigned to a given Account ID, to be retrieved later.

## Terminology

- **Conversations**: Defined as having a initiator and recipient with a list of messages.  
  Conversations can be marked as active or inactive. If the Conversation is active, it is
  added to the ActiveConversations StorageMap for both participants.

- **Convo ID**: A conversation id is how conversations are identified. It is the recipients and senders addresses hashed (right now, via SHA3-256), and is intended to be deterministic but unique.

- **Messages**: Defined as having a sender and receiver of a particular string of text.

- **Usernames**: Usernames are simple UTF-8 strings assigned to Account IDs for the purpose of human-readable identification
  of on-chain addresses.

## Testing & building

To test this repository, simply clone it and run:

```
cargo test --package pallet-uke --features runtime-benchmarks
```

To build it, run:

```
cargo build --release
```

## Implementation & Usage

To add this to an existing runtime, you may utilize the Substrate Node Template.

1.  Add the following in `runtime/Cargo.toml` under `[dependencies]`:

```rust
pallet-uke = { version = "4.0.0-dev", default-features = false, git = "https://github.com/Uke-Messaging/uke-pallet.git", branch = "main" }
```

2. And in `std` in that same `Cargo.toml`, add:

```rust
"pallet-uke/std",
```

3. In `runtime-benchmarks`, add:

```rust
"pallet-uke/runtime-benchmarks",
```

4. In `try-runtime `, add:

```rust
"pallet-uke/try-runtime",
```

5. Finally, add the pallet in the runtime as follows in `runtime/src/lib.rs`:

```rust
// Import the pallet
pub use pallet_uke;
```

```rust
// Implement the config
impl pallet_uke::Config for Runtime {
	type Event = Event;
	type MaxUsernameLength = ConstU32<16>;
	type MaxConvoIdLength = ConstU32<64>;
	type WeightInfo = pallet_uke::weights::PalletWeight<Runtime>;
}
```

Make sure to add it to the `construct_runtime!` macro:

```rust
construct_runtime!(
	pub struct Runtime
	where
		Block = Block,
		NodeBlock = opaque::Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
        ...
		Uke: pallet_uke,
        ...
    }
```

Lastly, add it in `mod benches`:

```rust
#[cfg(feature = "runtime-benchmarks")]
mod benches {
	define_benchmarks!(
        ...
		[pallet_uke, Uke]
        ...
	);
}
```

You can then run the node as normal and utilize the pallet as needed.

License: Apache 2.0


