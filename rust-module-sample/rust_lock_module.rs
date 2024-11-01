//! Rust out-of-tree sample

use kernel::prelude::*;
use kernel::sync::{new_mutex, new_spinlock, Mutex, SpinLock};

module! {
    type: RustLockModule,
    name: "rust_lock_module",
    author: "Rust for Linux Contributors",
    description: "Rust locks sample",
    license: "GPL",
}

struct Inner {
    a: u32,
    b: u32,
}

#[pin_data]
struct ExampleMtx {
    #[pin]
    d: Mutex<Inner>,
}

#[pin_data]
struct ExampleSpin {
    #[pin]
    d: SpinLock<Inner>,
}

impl ExampleMtx {
    fn new() -> impl PinInit<Self> {
        let (a, b) = (5, 10);
        pr_info!("Mutex Init Values - a: {}, b: {}\n", a, b);
        
        pin_init!(Self {
            d <- new_mutex!(Inner { a: a, b: b }),
        })
    }
}

impl ExampleSpin {
    fn new() -> impl PinInit<Self> {
        let (a, b) = (3, 6);
        pr_info!("Spin Init Values - a: {}, b: {}\n", a, b);
        
        pin_init!(Self {
            d <- new_spinlock!(Inner { a: a, b: b }),
        })
    }
}

struct RustLockModule {
    example_mtx: Pin<KBox<ExampleMtx>>,
    example_spin: Pin<KBox<ExampleSpin>>,
}

impl kernel::Module for RustLockModule {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        pr_info!("Rust module lock sample (init)\n");

        let example_mtx = KBox::pin_init(ExampleMtx::new(), GFP_KERNEL)?;
        let example_spin = KBox::pin_init(ExampleSpin::new(), GFP_KERNEL)?;

        Ok(RustLockModule {example_mtx, example_spin})
    }
}

impl RustLockModule {

    fn example_mtx_operation(m: &Pin<KBox<ExampleMtx>>) {
        let mut guard = m.d.lock();
        guard.a += 10;
        guard.b += 20;
    }

    fn example_spin_operation(m: &Pin<KBox<ExampleSpin>>) {
        let mut guard = m.d.lock();
        guard.a += 9;
        guard.b += 14;
    }
}

impl Drop for RustLockModule {
    fn drop(&mut self) {
        Self::example_mtx_operation(&self.example_mtx);
        Self::example_spin_operation(&self.example_spin);

        if let Some(guard) = self.example_mtx.d.try_lock() {
            pr_info!("Mutex Values - a: {}, b: {}\n", guard.a, guard.b);
        }

        if let Some(guard) = self.example_spin.d.try_lock() {
            pr_info!("Spin Values - a: {}, b: {}\n", guard.a, guard.b);
        }

        pr_info!("Rust lock sample (exit)\n");
    }
}
