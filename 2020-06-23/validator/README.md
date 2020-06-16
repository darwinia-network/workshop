# Validator

| Topic     | Speaker | Date       |
|-----------|---------|------------|
| Validator | Mercury | 2020-06-23 |


## Keywords

### Session

A session is a period of time that has a constant set of validators. Validators can only join or exit the validator set at a session change. It is measured in block numbers. The block where a session is ended is determined by the ShouldEndSession trait. When the session is ending, a new validator set can be chosen by OnSessionEnding implementations.

```rust
pub const MILLISECS_PER_BLOCK: Moment = 6000;
pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
pub const HOURS: BlockNumber = 60 * MINUTES;
pub const BLOCKS_PER_SESSION: BlockNumber = 1 * HOURS; // 1 hour, 600 blocks
```

### Era

A definable period, expressed as a range of block numbers, where a transaction may validly be included in a block. Eras are a backstop against transaction replay attacks in the case that an account is reaped and its (replay-protecting) nonce is reset to zero. Eras are efficiently expressible in transactions and cost only two bytes.

```rust
pub const SESSIONS_PER_ERA: SessionIndex = 6; // 6 hours, 3600 blocks
```

### Vote

Voting system where voter can vote for as many candidates as desired. The candidate with highest overall amount of votes wins. Notably:

+ voting for all candidates is exactly equivalent to voting for none; and
+ it is possible to vote "against" a single candidate by voting for all other candidates.

### Power

```rust
/// Power is a mixture of ring and kton
/// For *RING* power = ring_ratio * POWER_COUNT / 2
/// For *KTON* power = kton_ratio * POWER_COUNT / 2
pub fn currency_to_power<S: TryInto<Balance>>(active: S, pool: S) -> Power {
	(Perquintill::from_rational_approximation(
		active.saturated_into::<Balance>(),
		pool.saturated_into::<Balance>().max(1),
	) * (T::TotalPower::get() as Balance / 2)) as _
}

/// The total power that can be slashed from a stash account as of right now.
pub fn power_of(stash: &T::AccountId) -> Power {
	// Weight note: consider making the stake accessible through stash.
	Self::bonded(stash)
		.and_then(Self::ledger)
		.map(|l| {
			Self::currency_to_power::<_>(l.active_ring, Self::ring_pool())
				+ Self::currency_to_power::<_>(l.active_kton, Self::kton_pool())
		})
		.unwrap_or_default()
}
```

### Reward

```rust
// code snippet
```

### Slash

```rust
// code snippet
```

### ROI

```rust
// expand reward
```
收益回报率

## How to become a validator?

+ [official link][0]

## FAQ

+ Epoch session，
+ slash/reward是怎么回事
+ 为什么每个session出块量是不固定的
+ 投票是怎么计算的，什么条件下可以当选，
+ 从投票人角度怎么评价一个validator是稳定的，是值得投票的，
+ ROI应该如何计算


[0]: https://docs.darwinia.network/docs/en/crab-tut-validator
