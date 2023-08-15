/*!
Utilities for initializing Lean's runtime
*/
use parking_lot::Mutex;

/// A convenience mutex, since [`lean_initialize_runtime_module`] and [`lean_initialize`] are *not* thread-safe.
///
/// It is convention to hold this when initializing Lean's runtime, or Lean modules, to make sure only one thread at a time is doing so.
/// This is used in this library to safely implement tests, but it is the user's responsibility to uphold thread-safety when using this API.
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

/// A helper function to call [`lean_initialize_runtime_module`] while holding the [`LEAN_INIT_MUTEX`].
///
/// This is equivalent to writing
/// ```rust
/// # use lean_sys::*;
/// unsafe {
///     let guard = LEAN_INIT_MUTEX.lock();
///     lean_initialize_runtime_module();
/// }
/// ```
//TODO: is this safe?
pub unsafe fn lean_initialize_runtime_module_locked() {
    let _guard = LEAN_INIT_MUTEX.lock();
    lean_initialize_runtime_module();
}

/// A helper function to call [`lean_initialize`] while holding the [`LEAN_INIT_MUTEX`].///
/// This is equivalent to writing
/// ```rust
/// # use lean_sys::*;
/// unsafe {
///     let guard = LEAN_INIT_MUTEX.lock();
///     lean_initialize();
/// }
/// ```
//TODO: is this safe?
pub unsafe fn lean_initialize_locked() {
    let _guard = LEAN_INIT_MUTEX.lock();
    lean_initialize();
}

extern "C" {
    pub fn lean_initialize_runtime_module();
    pub fn lean_initialize();
}
