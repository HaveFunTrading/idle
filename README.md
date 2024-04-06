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