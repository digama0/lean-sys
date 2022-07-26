/*!
Utilities for initializing Lean's runtime
*/
use parking_lot::Mutex;

/// A convenience mutex, since [`lean_initialize_runtime_module`] and [`lean_initialize`] are *not* thread-safe.
/// 
/// It is convention to hold this when initializing Lean's runtime, or Lean modules, to make sure only one thread at a time is doing so.
/// 
/// # Examples
/// ```rust
/// # use lean_sys::*;
/// unsafe {
///     let guard = LEAN_INIT_MUTEX.lock();
///     lean_initialize_runtime_module();
///     // LEAN_INIT_MUTEX is unlocked here when `guard` goes out of scope
/// }
/// let big_nat = unsafe { lean_uint64_to_nat(u64::MAX) };
/// assert!(!lean_is_scalar(big_nat));
/// ```
pub static LEAN_INIT_MUTEX: Mutex<()> = Mutex::new(());

#[link(name = "leanshared")]
extern "C" {
    pub fn lean_initialize_runtime_module();
    pub fn lean_initialize();
}
