/*!
Utilities for initializing Lean's runtime
*/

pub struct LibcMutex {
    mutex: libc::pthread_mutex_t,
}

unsafe impl lock_api::RawMutex for LibcMutex {
    const INIT: Self = Self {
        mutex: libc::PTHREAD_MUTEX_INITIALIZER,
    };

    type GuardMarker = lock_api::GuardNoSend;

    fn lock(&self) {
        unsafe {
            libc::pthread_mutex_lock(&self.mutex as *const _ as *mut _);
        }
    }

    fn try_lock(&self) -> bool {
        unsafe { libc::pthread_mutex_trylock(&self.mutex as *const _ as *mut _) == 0 }
    }

    unsafe fn unlock(&self) {
        libc::pthread_mutex_unlock(&self.mutex as *const _ as *mut _);
    }
}

pub type Mutex<T> = lock_api::Mutex<LibcMutex, T>;
pub type MutexGuard<'a, T> = lock_api::MutexGuard<'a, LibcMutex, T>;

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
pub static LEAN_INIT_MUTEX: Mutex<bool> = Mutex::new(false);

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

#[cfg(test)]
pub(crate) mod test {
    use crate::{lean_initialize_runtime_module, LEAN_INIT_MUTEX};
    extern crate std;

    #[test]
    fn basic_mutex() {
        let mutex = std::sync::Mutex::new(0);
        std::thread::scope(|s| {
            for i in 0..20 {
                let mutex = &mutex;
                s.spawn(move || {
                    let mut guard = mutex.lock().unwrap();
                    *guard += i;
                });
            }
        });
        std::assert_eq!(mutex.into_inner().unwrap(), 190);
    }

    static INITIALIZED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

    pub(crate) fn initialize_runtime_module_for_tests() {
        let _guard = LEAN_INIT_MUTEX.lock();
        if !INITIALIZED.load(std::sync::atomic::Ordering::SeqCst) {
            unsafe {
                lean_initialize_runtime_module();
            }
            INITIALIZED.store(true, std::sync::atomic::Ordering::SeqCst);
        }
    }
}
