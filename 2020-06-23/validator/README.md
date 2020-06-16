# Validator

| Topic     | Speaker |
|-----------|---------|
| Validator | Mercury |


## Keywords

### Era

A definable period, expressed as a range of block numbers, where a transaction may validly be included in a block. Eras are a backstop against transaction replay attacks in the case that an account is reaped and its (replay-protecting) nonce is reset to zero. Eras are efficiently expressible in transactions and cost only two bytes.

### Slash

Punishment

### Reward

Just reward

### Session

A session is a period of time that has a constant set of validators. Validators can only join or exit the validator set at a session change. It is measured in block numbers. The block where a session is ended is determined by the ShouldEndSession trait. When the session is ending, a new validator set can be chosen by OnSessionEnding implementations.

### Vote

Voting system where voter can vote for as many candidates as desired. The candidate with highest overall amount of votes wins. Notably:

+ voting for all candidates is exactly equivalent to voting for none; and
+ it is possible to vote "against" a single candidate by voting for all other candidates.

### ROI

收益回报率


## FAQ

+ Epoch session，
+ slash/reward是怎么回事
+ 为什么每个session出块量是不固定的
+ 投票是怎么计算的，什么条件下可以当选，
+ 从投票人角度怎么评价一个validator是稳定的，是值得投票的，
+ ROI应该如何计算
