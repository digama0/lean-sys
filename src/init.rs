/*!
Utilities for initializing Lean's runtime
*/

extern "C" {
    pub fn lean_initialize_runtime_module();
    pub fn lean_initialize();
}

#[cfg(test)]
pub(crate) mod test {
    use crate::lean_initialize_runtime_module;
    extern crate std;

    static INITIALIZED: std::sync::Mutex<bool> = std::sync::Mutex::new(false);

    pub(crate) fn initialize_runtime_module_for_tests() {
        let mut guard = INITIALIZED.lock().unwrap();
        if !*guard {
            unsafe {
                lean_initialize_runtime_module();
            }
            *guard = true;
        }
    }
}
