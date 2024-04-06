[![Build Status](https://img.shields.io/endpoint.svg?url=https%3A%2F%2Factions-badge.atrox.dev%2Fhavefuntrading%2Fidle%2Fbadge%3Fref%3Dmain&style=flat&label=build&logo=none)](https://actions-badge.atrox.dev/havefuntrading/idle/goto?ref=main)
[![Crates.io](https://img.shields.io/crates/v/idle.svg)](https://crates.io/crates/idle)
[![Documentation](https://docs.rs/idle/badge.svg)](https://docs.rs/idle/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

Collection of idle strategies to be used by thread(s) when they have no work to perform. 
Inspired by [IdleStrategy](https://github.com/real-logic/agrona/blob/master/agrona/src/main/java/org/agrona/concurrent/IdleStrategy.java) from [Agrona](https://github.com/real-logic/agrona/tree/master).

```rust
use std::time::Duration;
use idle::IdleStrategy;

let idle = IdleStrategy::Sleep(Duration::from_millis(1));
loop {
    // application logic
    idle.idle(0);
}
```