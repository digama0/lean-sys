/*!
Utilities for initializing Lean threads
*/

extern "C" {
    pub fn lean_initialize_thread();
    pub fn lean_finalize_thread();
}
