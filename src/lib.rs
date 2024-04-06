//! Collection of idle strategies to be used by thread(s) when they have no work to perform.
//!
//! # Examples
//!
//! ```no_run
//! use std::time::Duration;
//! use idle::IdleStrategy;
//!
//! let idle = IdleStrategy::Sleep(Duration::from_millis(1));
//! loop {
//!     // application logic
//!     idle.idle(0);
//! }
//!
//! ```
//!

use std::hint;
use std::time::Duration;

/// Collection of idle strategies to be used by thread(s) when they have no work to perform.
/// Requires `work_count` to be passed that for some strategies is used as hint and ignored otherwise.
///
/// # Examples
///
/// ```no_run
/// use std::time::Duration;
/// use idle::IdleStrategy;
///
/// let idle = IdleStrategy::Sleep(Duration::from_millis(1));
/// loop {
///     // application logic
///     idle.idle(0);
/// }
/// ```
#[derive(Debug, Copy, Clone)]
pub enum IdleStrategy {
    /// Does absolutely nothing. Use if you require the lowest latency at the cost of burning the CPU.
    NoOp,
    /// Emits machine specific instruction to signal it's in busy spin mode if no work done in the current cycle.
    BusySpin,
    /// Perform the `std::thread::sleep` if there is no work done in the current cycle.
    Sleep(Duration),
}

impl IdleStrategy {
    #[inline(always)]
    pub fn idle(&self, work_count: usize) {
        match *self {
            IdleStrategy::NoOp => {}
            IdleStrategy::BusySpin => {
                if work_count == 0 {
                    hint::spin_loop()
                }
            }
            IdleStrategy::Sleep(duration) => {
                if work_count == 0 {
                    std::thread::sleep(duration)
                }
            }
        }
    }
}
