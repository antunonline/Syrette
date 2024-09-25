use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering::SeqCst;
use syrette_macros::declare_interface;

pub trait SomeTrait : Send + Sync {
    fn trait_increment(&self);
}

pub struct SomeSingleton {
    cntr: AtomicU32
}


#[syrette::injectable(async=true)]
impl SomeSingleton {
    pub fn new() -> Self {
        Self {
            cntr: AtomicU32::new(0)
        }
    }

    pub fn struct_increment(&self) {
        println!("{}", self.cntr.fetch_add(1, SeqCst));
    }
}

impl SomeTrait for SomeSingleton {
    fn trait_increment(&self) {
        println!("{}", self.cntr.fetch_add(1, SeqCst));
    }
}

declare_interface!(SomeSingleton -> SomeTrait, threadsafe_sharable=true);