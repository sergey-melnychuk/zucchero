//! # zucchero
//!
//! A minimal macro to create a globally shared singleton.

/// Define a global singleton object and expose accessor function
#[macro_export]
macro_rules! init {
    ($type:ty, $func_name:ident) => {
        mod $func_name {
            use super::*;
            use once_cell::sync::OnceCell;
            use std::sync::Mutex;

            struct Global<T> {
                cell: OnceCell<Mutex<T>>,
            }

            impl<T: Default + Send + Sync + 'static> Global<T> {
                #[allow(clippy::new_without_default)]
                const fn new() -> Self {
                    Self {
                        cell: OnceCell::new(),
                    }
                }

                fn apply<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
                    let mut guard = self
                        .cell
                        .get_or_init(|| Mutex::new(T::default()))
                        .lock()
                        .unwrap();
                    f(&mut *guard)
                }
            }

            static GLOBAL: Global<$type> = Global::new();

            pub fn call<R>(f: impl FnOnce(&mut $type) -> R) -> R {
                GLOBAL.apply(f)
            }
        }

        pub use $func_name::call as $func_name;
    };
}
