## Setup

### Auto
```sh
curl https://getsubstrate.io -sSf | bash -s -- --fast
```

### Manual (recommend)
```sh
curl https://sh.rustup.rs -sSf | sh

# fix hex-literal compile error
rustup default nightly-2019-07-14

rustup target add wasm-32-unknown-unknown
cargo install --git https://github.com/alexcrichton/wasm-gc

brew intsall cmake git openssl pkg-config llvm
nix-env -i cmake git openssl pkg-config llvm
# optional
nix-env -i nodejs yarn
```

```sh
substrate-node-new erc20-demo itering
cd erc20-demo
./target/release/erc20-demo --dev

# optional
substrate-ui-new erc20-demo
cd erc20-demo-ui
yarn run dev
```

## Initialize

go to `erc20-demo` folder:

```sh
touch runtime/src/erc20_demo.rs
```

---

`runtime/src/lib.rs` > `#![cfg_attr]` > `mod`:

```rust
// del `#![cfg_attr(not(feature = "std"), feature(alloc))]`
// the feature `alloc` has been stable since 1.36.0 and no longer requires an attribute to enable

// replace `mod template;`
mod erc20_demo;
```

---

`runtime/src/erc20_demo.rs` > import > `Trait`:

```rust
use parity_codec::Codec;
use rstd::prelude::*;
use runtime_primitives::traits::{As, CheckedAdd, CheckedSub, Member, SimpleArithmetic};
use support::{
    decl_event, decl_module, decl_storage, dispatch::Result, ensure, Parameter, StorageMap,
    StorageValue,
};
use system::ensure_signed;

pub trait Trait: system::Trait {
    type TokenBalance: Parameter
        + Member
        + SimpleArithmetic
        + Codec
        + Default
        + Copy
        + As<usize>
        + As<u64>;
}
```

---

`runtime/src/lib.rs` > `erc20_demo::Trait` > `decl_storage!`:

```rust
impl erc20_demo::Trait for Runtime {
    type TokenBalance = u128;
}

decl_storage! {
    trait Store for Module<T: Trait> as Erc20Demo {
        Init get(is_init): bool;
        Owner get(owner) config(): T::AccountId;
        TotalSupply get(total_supply) config(): T::TokenBalance;
        Name get(name) config(): Vec<u8>;
        Ticker get(ticker) config(): Vec<u8>;
        BalanceOf get(balance_of): map T::AccountId => T::TokenBalance;
        Allowance get(allowance): map (T::AccountId, T::AccountId) => T::TokenBalance;
    }
}
```

Refs:
- `GenesisConfig` -> [link](https://substrate.dev/docs/en/runtime/types/genesisconfig-struct)
- `StorageMap` -> [link](https://substrate.dev/rustdocs/v1.0/srml_support/storage/trait.StorageMap.html)
- `StorageValue` -> [link](https://substrate.dev/rustdocs/v1.0/srml_support/storage/trait.StorageValue.html)

---

`runtime/src/lib.rs` > `construct_runtime!`:

```rust
construct_runtime!(
    pub enum Runtime with Log(InternalLog: DigestItem<Hash, AuthorityId, AuthoritySignature>) 
        where
            Block = Block,
            NodeBlock = opaque::Block,
            UncheckedExtrinsic = UncheckedExtrinsic
    {
        System: system::{default, Log(ChangesTrieRoot)},
        Timestamp: timestamp::{Module, Call, Storage, Config<T>, Inherent},
        Consensus: consensus::{Module, Call, Storage, Config<T>, Log(AuthoritiesChange), Inherent},
        Aura: aura::{Module},
        Indices: indices,
        Balances: balances,
        Sudo: sudo,
        Erc20Demo: erc20_demo::{Module, Call, Storage, Event<T>, Config<T>},
    }
);
```

Refs:
- `construct_runtime!` -> [link](https://substrate.dev/docs/en/runtime/macros/construct_runtime)
- `Call` -> [link](https://substrate.dev/docs/en/runtime/types/call-enum)
- `Event` -> [link](https://substrate.dev/docs/en/runtime/types/event-enum)

---

`src/chain_spec.rs` > import > `testnet_genesis( ... )`:

```rust
use erc20_demo_runtime::{ 
    ... ,
    Erc20DemoConfig
};

fn testnet_genesis( ... ) -> GenesisConfig {
    GenesisConfig {
        ... ,
        erc20_demo: Some(Erc20DemoConfig {
            owner: account_key("Alice"),
            total_supply: 21000000,
            name: "SubstrateDemoToken".as_bytes().into(),
            ticker: "SDT".as_bytes().into() 
        }),
    }
}
```

Refs:
- `account_key()` -> `src/chain_spec.rs` line 33
- `sr25519::Pair` -> [link](https://substrate.dev/rustdocs/v1.0/sr_io/sr25519/struct.Pair.html)

---

`runtime/src/erc20_demo.rs` > `decl_module!` > `init()`:

```rust
decl_module! {
    pub struct Module<T: Trait> for enum Call
        where
            origin: T::Origin
    {
        fn init(origin) -> Result {
            let sender = ensure_signed(origin)?;

            ensure!(Self::is_init() == false, "Already initialized.");
            ensure!(Self::owner() == sender, "Only owner can initalized.");

            <BalanceOf<T>>::insert(sender.clone(), Self::total_supply());
            <Init<T>>::put(true);

            Ok(())
        }
    }
}
```

## Summary

Now, `runtime/src/erc20_demo.rs` should look like:

```rust
use parity_codec::Codec;
use rstd::prelude::*;
use runtime_primitives::traits::{As, CheckedAdd, CheckedSub, Member, SimpleArithmetic};
use support::{
    decl_event, decl_module, decl_storage, dispatch::Result, ensure, Parameter, StorageMap,
    StorageValue,
};
use system::ensure_signed;

pub trait Trait: system::Trait {
    type TokenBalance: Parameter
        + Member
        + SimpleArithmetic
        + Codec
        + Default
        + Copy
        + As<usize>
        + As<u64>;
}

decl_storage! {
    trait Store for Module<T: Trait> as Erc20Demo {
        Init get(is_init): bool;
        Owner get(owner) config(): T::AccountId;
        TotalSupply get(total_supply) config(): T::TokenBalance;
        Name get(name) config(): Vec<u8>;
        Ticker get(ticker) config(): Vec<u8>;
        BalanceOf get(balance_of): map T::AccountId => T::TokenBalance;
        Allowance get(allowance): map (T::AccountId, T::AccountId) => T::TokenBalance;
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call
        where
            origin: T::Origin
    {
        fn init(origin) -> Result {
            let sender = ensure_signed(origin)?;

            ensure!(Self::is_init() == false, "Already initialized.");
            ensure!(Self::owner() == sender, "Only owner can initalized.");

            <BalanceOf<T>>::insert(sender.clone(), Self::total_supply());
            <Init<T>>::put(true);

            Ok(())
        }
    }
}
```

## Implement

`runtime/src/erc20_demo.rs` > `decl_module!` > `transfer()`:

```rust
decl_module! {
    pub struct Module<T: Trait> for enum Call
        where
            origin: T::Origin
    {
        fn init( ... ) -> Result { ... }

        fn transfer(
            origin,
            to: T::AccountId, 
            #[compact] value: T::TokenBalance
        ) -> Result 
        {
            let sender = ensure_signed(origin)?;
            Self::int_transfer(sender, to, value)
        }
    }
}
```

---

`runtime/src/erc20_demo.rs` > `impl module` > `int_transfer()`:

```rust
impl<T: Trait> Module<T> {
    fn int_transfer(
        from: T::AccountId, 
        to: T::AccountId, 
        value: T::TokenBalance
    ) -> Result 
    {
        ensure!(
            <BalanceOf<T>>::exists(from.clone()),
            "Account does not own this token."
        );

        let sender_balance = {
            let sender_balance = Self::balance_of(from.clone());
            ensure!(sender_balance >= value, "Not enough balance.");

            sender_balance
                .checked_sub(&value)
                .ok_or("overflow in calculating balance")?
        };
        let receiver_balance = Self::balance_of(to.clone())
            .checked_add(&value)
            .ok_or("overflow in calculating balance")?;

        <BalanceOf<T>>::insert(from.clone(), sender_balance);
        <BalanceOf<T>>::insert(to.clone(), receiver_balance);

        // TODO deposit event

        Ok(())
    }
}
```

---

`runtime/src/erc20_demo.rs` > `decl_module!` > `approve()` > `transfer_from()`:

```rust
decl_module! {
    pub struct Module<T: Trait> for enum Call
        where
            origin: T::Origin
    {
        fn init( ... ) -> Result { ... }

        fn transfer( ... ) -> Result { ... }

        fn approve(
            origin, 
            spender: T::AccountId, 
            #[compact] value: T::TokenBalance
        ) -> Result 
        {
            let sender = ensure_signed(origin)?;

            ensure!(<BalanceOf<T>>::exists(&sender), "Account does not own this token.");

            let allowance = Self::allowance((sender.clone(), spender.clone()));
            let updated_allowance = allowance.checked_add(&value).ok_or("overflow in calculating allowance")?;

            <Allowance<T>>::insert((sender.clone(), spender.clone()), updated_allowance);

            // TODO deposit event

            Ok(())
        }

        fn transfer_from(
            origin, 
            from: T::AccountId, 
            to: T::AccountId,
            #[compact] value: T::TokenBalance
        ) -> Result 
        {
            ensure!(<Allowance<T>>::exists((from.clone(), to.clone())), "Allowance does not exist.");
            
            let allowance = Self::allowance((from.clone(), to.clone()));
            ensure!(allowance >= value, "Not enough allowance.");

            let updated_allowance = allowance.checked_sub(&value).ok_or("overflow in calculating allowance")?;

            <Allowance<T>>::insert((from.clone(), to.clone()), updated_allowance);

            // TODO deposit event

            Self::int_transfer(from, to, value)
        }
    }
}
```

## Summary

Now, `runtime/src/erc20_demo.rs` should look like:

```rust
use parity_codec::Codec;
use rstd::prelude::*;
use runtime_primitives::traits::{As, CheckedAdd, CheckedSub, Member, SimpleArithmetic};
use support::{
    decl_event, decl_module, decl_storage, dispatch::Result, ensure, Parameter, StorageMap,
    StorageValue,
};
use system::ensure_signed;

pub trait Trait: system::Trait {
    type TokenBalance: Parameter
        + Member
        + SimpleArithmetic
        + Codec
        + Default
        + Copy
        + As<usize>
        + As<u64>;
}

decl_storage! {
    trait Store for Module<T: Trait> as Erc20Demo {
        Init get(is_init): bool;
        Owner get(owner) config(): T::AccountId;
        TotalSupply get(total_supply) config(): T::TokenBalance;
        Name get(name) config(): Vec<u8>;
        Ticker get(ticker) config(): Vec<u8>;
        BalanceOf get(balance_of): map T::AccountId => T::TokenBalance;
        Allowance get(allowance): map (T::AccountId, T::AccountId) => T::TokenBalance;
    }
}

impl<T: Trait> Module<T> {
    fn int_transfer(
        from: T::AccountId, 
        to: T::AccountId, 
        value: T::TokenBalance
    ) -> Result 
    {
        ensure!(
            <BalanceOf<T>>::exists(from.clone()),
            "Account does not own this token."
        );

        let sender_balance = {
            let sender_balance = Self::balance_of(from.clone());
            ensure!(sender_balance >= value, "Not enough balance.");

            sender_balance
                .checked_sub(&value)
                .ok_or("overflow in calculating balance")?
        };
        let receiver_balance = Self::balance_of(to.clone())
            .checked_add(&value)
            .ok_or("overflow in calculating balance")?;

        <BalanceOf<T>>::insert(from.clone(), sender_balance);
        <BalanceOf<T>>::insert(to.clone(), receiver_balance);

        // TODO deposit event

        Ok(())
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call
        where
            origin: T::Origin
    {
        fn init(origin) -> Result {
            let sender = ensure_signed(origin)?;

            ensure!(Self::is_init() == false, "Already initialized.");
            ensure!(Self::owner() == sender, "Only owner can initalized.");

            <BalanceOf<T>>::insert(sender.clone(), Self::total_supply());
            <Init<T>>::put(true);

            Ok(())
        }

        fn transfer(
            origin,
            to: T::AccountId, 
            #[compact] value: T::TokenBalance
        ) -> Result 
        {
            let sender = ensure_signed(origin)?;
            Self::int_transfer(sender, to, value)
        }

        fn approve(
            origin, 
            spender: T::AccountId, 
            #[compact] value: T::TokenBalance
        ) -> Result 
        {
            let sender = ensure_signed(origin)?;

            ensure!(<BalanceOf<T>>::exists(&sender), "Account does not own this token.");

            let allowance = Self::allowance((sender.clone(), spender.clone()));
            let updated_allowance = allowance.checked_add(&value).ok_or("overflow in calculating allowance")?;

            <Allowance<T>>::insert((sender.clone(), spender.clone()), updated_allowance);

            // TODO deposit event

            Ok(())
        }

        fn transfer_from(
            origin, 
            from: T::AccountId, 
            to: T::AccountId,
            #[compact] value: T::TokenBalance
        ) -> Result 
        {
            ensure!(<Allowance<T>>::exists((from.clone(), to.clone())), "Allowance does not exist.");
            
            let allowance = Self::allowance((from.clone(), to.clone()));
            ensure!(allowance >= value, "Not enough allowance.");

            let updated_allowance = allowance.checked_sub(&value).ok_or("overflow in calculating allowance")?;

            <Allowance<T>>::insert((from.clone(), to.clone()), updated_allowance);

            // TODO deposit event

            Self::int_transfer(from, to, value)
        }
    }
}
```

## Deposit event

`runtime/src/erc20_demo.rs` > `Event`:

```rust
pub trait Trait: system::Trait {
    type TokenBalance ...;
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}
```

---

`runtime/src/lib.rs` > `Event`:

```rust
impl erc20_demo::Trait for Runtime {
    type TokenBalance ... ;
    type Event = Event;
}
```

---

`runtime/src/erc20_demo.rs` > `impl module` > `int_transfer()`:

```rust
impl<T: Trait> Module<T> {
    fn int_transfer( ... ) -> Result {
        ...

        Self::deposit_event(RawEvent::Transfer(from, to, value));

        Ok(())
    }
}
```

---

`runtime/src/erc20_demo.rs` > `decl_module!` > `deposit_event()` > `approve()` > `transfer_from()`:

```rust
decl_module! {
    pub struct Module<T: Trait> for enum Call
        where
            origin: T::Origin
    {
        fn deposit_event<T>() = default;

        fn init(...) -> Result { ... }

        fn transfer( ... ) -> Result { ... }

        fn approve( ... ) -> Result { 
            ...

            Self::deposit_event(RawEvent::Approval(sender, spender, value));

            Ok(())
        }

        fn transfer_from( ... ) -> Result {
            ...

            Self::deposit_event(RawEvent::Approval(from.clone(), to.clone(), value));

            Self::int_transfer(from, to, value)
        }
    }
}
```

---

`runtime/src/erc20_demo.rs` > `decl_event!`:

```rust
decl_event! {
    pub enum Event<T> 
        where 
            AccountId = <T as system::Trait>::AccountId,
            Balance = <T as Trait>::TokenBalance
    {
        Transfer(AccountId, AccountId, Balance),
        Approval(AccountId, AccountId, Balance),
    }
}
```

## Summary

Now,

`src/chain_spec.rs` should look like:

```rust
use erc20_demo_runtime::{
    AccountId, BalancesConfig, ConsensusConfig, Erc20DemoConfig, GenesisConfig, IndicesConfig,
    SudoConfig, TimestampConfig,
};
use primitives::{ed25519, sr25519, Pair};
use substrate_service;

use ed25519::Public as AuthorityId;

// Note this is the URL for the telemetry server
//const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = substrate_service::ChainSpec<GenesisConfig>;

/// The chain specification option. This is expected to come in from the CLI and
/// is little more than one of a number of alternatives which can easily be converted
/// from a string (`--chain=...`) into a `ChainSpec`.
#[derive(Clone, Debug)]
pub enum Alternative {
    /// Whatever the current runtime is, with just Alice as an auth.
    Development,
    /// Whatever the current runtime is, with simple Alice/Bob auths.
    LocalTestnet,
}

fn authority_key(s: &str) -> AuthorityId {
    ed25519::Pair::from_string(&format!("//{}", s), None)
        .expect("static values are valid; qed")
        .public()
}

fn account_key(s: &str) -> AccountId {
    sr25519::Pair::from_string(&format!("//{}", s), None)
        .expect("static values are valid; qed")
        .public()
}

impl Alternative {
    /// Get an actual chain config from one of the alternatives.
    pub(crate) fn load(self) -> Result<ChainSpec, String> {
        Ok(match self {
            Alternative::Development => ChainSpec::from_genesis(
                "Development",
                "dev",
                || {
                    testnet_genesis(
                        vec![authority_key("Alice")],
                        vec![account_key("Alice")],
                        account_key("Alice"),
                    )
                },
                vec![],
                None,
                None,
                None,
                None,
            ),
            Alternative::LocalTestnet => ChainSpec::from_genesis(
                "Local Testnet",
                "local_testnet",
                || {
                    testnet_genesis(
                        vec![authority_key("Alice"), authority_key("Bob")],
                        vec![
                            account_key("Alice"),
                            account_key("Bob"),
                            account_key("Charlie"),
                            account_key("Dave"),
                            account_key("Eve"),
                            account_key("Ferdie"),
                        ],
                        account_key("Alice"),
                    )
                },
                vec![],
                None,
                None,
                None,
                None,
            ),
        })
    }

    pub(crate) fn from(s: &str) -> Option<Self> {
        match s {
            "dev" => Some(Alternative::Development),
            "" | "local" => Some(Alternative::LocalTestnet),
            _ => None,
        }
    }
}

fn testnet_genesis(
    initial_authorities: Vec<AuthorityId>,
    endowed_accounts: Vec<AccountId>,
    root_key: AccountId,
) -> GenesisConfig {
    GenesisConfig {
        consensus: Some(ConsensusConfig {
            code: include_bytes!("../runtime/wasm/target/wasm32-unknown-unknown/release/erc20_demo_runtime_wasm.compact.wasm").to_vec(),
            authorities: initial_authorities.clone(),
        }),
        system: None,
        timestamp: Some(TimestampConfig {
            minimum_period: 5, // 10 second block time.
        }),
        indices: Some(IndicesConfig {
            ids: endowed_accounts.clone(),
        }),
        balances: Some(BalancesConfig {
            transaction_base_fee: 1,
            transaction_byte_fee: 0,
            existential_deposit: 500,
            transfer_fee: 0,
            creation_fee: 0,
            balances: endowed_accounts.iter().cloned().map(|k|(k, 1 << 60)).collect(),
            vesting: vec![],
        }),
        sudo: Some(SudoConfig {
            key: root_key,
        }),
        erc20_demo: Some(Erc20DemoConfig {
            owner: account_key("Alice"),
            total_supply: 21000000,
            name: "SubstrateDemoToken".as_bytes().into(),
            ticker: "SDT".as_bytes().into(),
        }),
    }
}
```

---

`runtime/src/lib.rs` should look like:

```rust
//! The Substrate Node Template runtime. This can be compiled with `#[no_std]`, ready for Wasm.

#![cfg_attr(not(feature = "std"), no_std)]
// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 256.
#![recursion_limit = "256"]

use client::{
    block_builder::api::{self as block_builder_api, CheckInherentsResult, InherentData},
    impl_runtime_apis, runtime_api,
};
use parity_codec::{Decode, Encode};
#[cfg(feature = "std")]
use primitives::bytes;
use primitives::{ed25519, sr25519, OpaqueMetadata};
use rstd::prelude::*;
use runtime_primitives::{
    create_runtime_str, generic,
    traits::{self, BlakeTwo256, Block as BlockT, NumberFor, StaticLookup, Verify},
    transaction_validity::TransactionValidity,
    ApplyResult,
};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "std")]
use version::NativeVersion;
use version::RuntimeVersion;

// A few exports that help ease life for downstream crates.
pub use balances::Call as BalancesCall;
pub use consensus::Call as ConsensusCall;
#[cfg(any(feature = "std", test))]
pub use runtime_primitives::BuildStorage;
pub use runtime_primitives::{Perbill, Permill};
pub use support::{construct_runtime, StorageValue};
pub use timestamp::BlockPeriod;
pub use timestamp::Call as TimestampCall;

/// The type that is used for identifying authorities.
pub type AuthorityId = <AuthoritySignature as Verify>::Signer;

/// The type used by authorities to prove their ID.
pub type AuthoritySignature = ed25519::Signature;

/// Alias to pubkey that identifies an account on the chain.
pub type AccountId = <AccountSignature as Verify>::Signer;

/// The type used by authorities to prove their ID.
pub type AccountSignature = sr25519::Signature;

/// A hash of some data used by the chain.
pub type Hash = primitives::H256;

/// Index of a block number in the chain.
pub type BlockNumber = u64;

/// Index of an account's extrinsic in the chain.
pub type Nonce = u64;

mod erc20_demo;

/// Opaque types. These are used by the CLI to instantiate machinery that don't need to know
/// the specifics of the runtime. They can then be made to be agnostic over specific formats
/// of data like extrinsics, allowing for them to continue syncing the network through upgrades
/// to even the core datastructures.
pub mod opaque {
    use super::*;

    /// Opaque, encoded, unchecked extrinsic.
    #[derive(PartialEq, Eq, Clone, Default, Encode, Decode)]
    #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
    pub struct UncheckedExtrinsic(#[cfg_attr(feature = "std", serde(with = "bytes"))] pub Vec<u8>);
    #[cfg(feature = "std")]
    impl std::fmt::Debug for UncheckedExtrinsic {
        fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(fmt, "{}", primitives::hexdisplay::HexDisplay::from(&self.0))
        }
    }
    impl traits::Extrinsic for UncheckedExtrinsic {
        fn is_signed(&self) -> Option<bool> {
            None
        }
    }
    /// Opaque block header type.
    pub type Header = generic::Header<
        BlockNumber,
        BlakeTwo256,
        generic::DigestItem<Hash, AuthorityId, AuthoritySignature>,
    >;
    /// Opaque block type.
    pub type Block = generic::Block<Header, UncheckedExtrinsic>;
    /// Opaque block identifier type.
    pub type BlockId = generic::BlockId<Block>;
    /// Opaque session key type.
    pub type SessionKey = AuthorityId;
}

/// This runtime version.
pub const VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: create_runtime_str!("erc20-demo"),
    impl_name: create_runtime_str!("erc20-demo"),
    authoring_version: 3,
    spec_version: 4,
    impl_version: 4,
    apis: RUNTIME_API_VERSIONS,
};

/// The version infromation used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
    NativeVersion {
        runtime_version: VERSION,
        can_author_with: Default::default(),
    }
}

impl system::Trait for Runtime {
    /// The identifier used to distinguish between accounts.
    type AccountId = AccountId;
    /// The lookup mechanism to get account ID from whatever is passed in dispatchers.
    type Lookup = Indices;
    /// The index type for storing how many extrinsics an account has signed.
    type Index = Nonce;
    /// The index type for blocks.
    type BlockNumber = BlockNumber;
    /// The type for hashing blocks and tries.
    type Hash = Hash;
    /// The hashing algorithm used.
    type Hashing = BlakeTwo256;
    /// The header digest type.
    type Digest = generic::Digest<Log>;
    /// The header type.
    type Header = generic::Header<BlockNumber, BlakeTwo256, Log>;
    /// The ubiquitous event type.
    type Event = Event;
    /// The ubiquitous log type.
    type Log = Log;
    /// The ubiquitous origin type.
    type Origin = Origin;
}

impl aura::Trait for Runtime {
    type HandleReport = ();
}

impl consensus::Trait for Runtime {
    /// The identifier we use to refer to authorities.
    type SessionKey = AuthorityId;
    // The aura module handles offline-reports internally
    // rather than using an explicit report system.
    type InherentOfflineReport = ();
    /// The ubiquitous log type.
    type Log = Log;
}

impl indices::Trait for Runtime {
    /// The type for recording indexing into the account enumeration. If this ever overflows, there
    /// will be problems!
    type AccountIndex = u32;
    /// Use the standard means of resolving an index hint from an id.
    type ResolveHint = indices::SimpleResolveHint<Self::AccountId, Self::AccountIndex>;
    /// Determine whether an account is dead.
    type IsDeadAccount = Balances;
    /// The uniquitous event type.
    type Event = Event;
}

impl timestamp::Trait for Runtime {
    /// A timestamp: seconds since the unix epoch.
    type Moment = u64;
    type OnTimestampSet = Aura;
}

impl balances::Trait for Runtime {
    /// The type for recording an account's balance.
    type Balance = u128;
    /// What to do if an account's free balance gets zeroed.
    type OnFreeBalanceZero = ();
    /// What to do if a new account is created.
    type OnNewAccount = Indices;
    /// The uniquitous event type.
    type Event = Event;

    type TransactionPayment = ();
    type DustRemoval = ();
    type TransferPayment = ();
}

impl sudo::Trait for Runtime {
    /// The uniquitous event type.
    type Event = Event;
    type Proposal = Call;
}

impl erc20_demo::Trait for Runtime {
    type TokenBalance = u128;
    type Event = Event;
}

construct_runtime!(
    pub enum Runtime with Log(InternalLog: DigestItem<Hash, AuthorityId, AuthoritySignature>) where
        Block = Block,
        NodeBlock = opaque::Block,
        UncheckedExtrinsic = UncheckedExtrinsic
    {
        System: system::{default, Log(ChangesTrieRoot)},
        Timestamp: timestamp::{Module, Call, Storage, Config<T>, Inherent},
        Consensus: consensus::{Module, Call, Storage, Config<T>, Log(AuthoritiesChange), Inherent},
        Aura: aura::{Module},
        Indices: indices,
        Balances: balances,
        Sudo: sudo,
        Erc20Demo: erc20_demo::{Module, Call, Storage, Event<T>, Config<T>},
    }
);

/// The type used as a helper for interpreting the sender of transactions.
type Context = system::ChainContext<Runtime>;
/// The address format for describing accounts.
type Address = <Indices as StaticLookup>::Source;
/// Block header type as expected by this runtime.
pub type Header = generic::Header<BlockNumber, BlakeTwo256, Log>;
/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
/// BlockId type as expected by this runtime.
pub type BlockId = generic::BlockId<Block>;
/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic =
    generic::UncheckedMortalCompactExtrinsic<Address, Nonce, Call, AccountSignature>;
/// Extrinsic type that has already been checked.
pub type CheckedExtrinsic = generic::CheckedExtrinsic<AccountId, Nonce, Call>;
/// Executive: handles dispatch to the various modules.
pub type Executive = executive::Executive<Runtime, Block, Context, Balances, AllModules>;

// Implement our runtime API endpoints. This is just a bunch of proxying.
impl_runtime_apis! {
    impl runtime_api::Core<Block> for Runtime {
        fn version() -> RuntimeVersion {
            VERSION
        }

        fn execute_block(block: Block) {
            Executive::execute_block(block)
        }

        fn initialize_block(header: &<Block as BlockT>::Header) {
            Executive::initialize_block(header)
        }

        fn authorities() -> Vec<AuthorityId> {
            panic!("Deprecated, please use `AuthoritiesApi`.")
        }
    }

    impl runtime_api::Metadata<Block> for Runtime {
        fn metadata() -> OpaqueMetadata {
            Runtime::metadata().into()
        }
    }

    impl block_builder_api::BlockBuilder<Block> for Runtime {
        fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyResult {
            Executive::apply_extrinsic(extrinsic)
        }

        fn finalize_block() -> <Block as BlockT>::Header {
            Executive::finalize_block()
        }

        fn inherent_extrinsics(data: InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
            data.create_extrinsics()
        }

        fn check_inherents(block: Block, data: InherentData) -> CheckInherentsResult {
            data.check_extrinsics(&block)
        }

        fn random_seed() -> <Block as BlockT>::Hash {
            System::random_seed()
        }
    }

    impl runtime_api::TaggedTransactionQueue<Block> for Runtime {
        fn validate_transaction(tx: <Block as BlockT>::Extrinsic) -> TransactionValidity {
            Executive::validate_transaction(tx)
        }
    }

    impl consensus_aura::AuraApi<Block> for Runtime {
        fn slot_duration() -> u64 {
            Aura::slot_duration()
        }
    }

    impl offchain_primitives::OffchainWorkerApi<Block> for Runtime {
        fn offchain_worker(n: NumberFor<Block>) {
            Executive::offchain_worker(n)
        }
    }

    impl consensus_authorities::AuthoritiesApi<Block> for Runtime {
        fn authorities() -> Vec<AuthorityId> {
            Consensus::authorities()
        }
    }
}
```

---

`runtime/src/erc20_demo.rs` should look like:

```rust
use parity_codec::Codec;
use rstd::prelude::*;
use runtime_primitives::traits::{As, CheckedAdd, CheckedSub, Member, SimpleArithmetic};
use support::{
    decl_event, decl_module, decl_storage, dispatch::Result, ensure, Parameter, StorageMap,
    StorageValue,
};
use system::ensure_signed;

pub trait Trait: system::Trait {
    type TokenBalance: Parameter
        + Member
        + SimpleArithmetic
        + Codec
        + Default
        + Copy
        + As<usize>
        + As<u64>;
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_storage! {
    trait Store for Module<T: Trait> as Erc20Demo {
        Init get(is_init): bool;
        Owner get(owner) config(): T::AccountId;
        TotalSupply get(total_supply) config(): T::TokenBalance;
        Name get(name) config(): Vec<u8>;
        Ticker get(ticker) config(): Vec<u8>;
        BalanceOf get(balance_of): map T::AccountId => T::TokenBalance;
        Allowance get(allowance): map (T::AccountId, T::AccountId) => T::TokenBalance;
    }
}

impl<T: Trait> Module<T> {
    fn int_transfer(
        from: T::AccountId, 
        to: T::AccountId, 
        value: T::TokenBalance
    ) -> Result 
    {
        ensure!(
            <BalanceOf<T>>::exists(from.clone()),
            "Account does not own this token."
        );

        {
            let sender_balance = {
                let sender_balance = Self::balance_of(from.clone());
                ensure!(sender_balance >= value, "Not enough balance.");

                sender_balance
                    .checked_sub(&value)
                    .ok_or("overflow in calculating balance")?
            };
            let receiver_balance = Self::balance_of(to.clone())
                .checked_add(&value)
                .ok_or("overflow in calculating balance")?;

            <BalanceOf<T>>::insert(from.clone(), sender_balance);
            <BalanceOf<T>>::insert(to.clone(), receiver_balance);
        }

        Self::deposit_event(RawEvent::Transfer(from, to, value));

        Ok(())
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call
        where
            origin: T::Origin
    {
        fn deposit_event<T>() = default;

        fn init(origin) -> Result {
            let sender = ensure_signed(origin)?;

            ensure!(Self::is_init() == false, "Already initialized.");
            ensure!(Self::owner() == sender, "Only owner can initalized.");

            <BalanceOf<T>>::insert(sender.clone(), Self::total_supply());
            <Init<T>>::put(true);

            Ok(())
        }

        fn transfer(
            origin,
            to: T::AccountId,
            #[compact] value: T::TokenBalance
        ) -> Result
        {
            let sender = ensure_signed(origin)?;
            Self::int_transfer(sender, to, value)
        }

        fn approve(
            origin,
            spender: T::AccountId,
            #[compact] value: T::TokenBalance
        ) -> Result
        {
            let sender = ensure_signed(origin)?;
            ensure!(<BalanceOf<T>>::exists(&sender), "Account does not own this token.");

            {
                let allowance = Self::allowance((sender.clone(), spender.clone()));
                let updated_allowance = allowance.checked_add(&value).ok_or("overflow in calculating allowance")?;

                <Allowance<T>>::insert((sender.clone(), spender.clone()), updated_allowance);
            }

            Self::deposit_event(RawEvent::Approval(sender, spender, value));

            Ok(())
        }

        fn transfer_from(
            origin,
            from: T::AccountId,
            to: T::AccountId,
            #[compact] value: T::TokenBalance
        ) -> Result
        {
            let sender = ensure_signed(origin)?;
            ensure!(<Allowance<T>>::exists((from.clone(), sender.clone())), "Allowance does not exist.");

            let allowance = Self::allowance((from.clone(), sender.clone()));
            ensure!(allowance >= value, "Not enough allowance.");

            {
                let updated_allowance = allowance - value;
                // let updated_allowance = allowance.checked_sub(&value).ok_or("overflow in calculating allowance")?;

                <Allowance<T>>::insert((from.clone(), sender.clone()), updated_allowance);
            }

            // Self::deposit_event(RawEvent::Approval(from.clone(), sender.clone(), value));

            Self::int_transfer(from, to, value)
        }
    }
}

decl_event! {
    pub enum Event<T>
        where
            AccountId = <T as system::Trait>::AccountId,
            Balance = <T as Trait>::TokenBalance
    {
        Transfer(AccountId, AccountId, Balance),
        Approval(AccountId, AccountId, Balance),
    }
}
```