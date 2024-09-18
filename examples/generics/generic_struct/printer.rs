use std::fmt::Display;
use std::marker::PhantomData;

pub struct GenericStructPrinter<Msg: Send + Display> {
    phantom_data: PhantomData<Msg>,
}


impl<Msg: Send + Display> GenericStructPrinter<Msg> {
    pub fn print(&self, msg: &Msg) {
        println!("{}", msg);
    }
}


impl Default for GenericStructPrinter<String> {
    fn default() -> Self {
        Self {
            phantom_data: PhantomData::default()
        }
    }
}

impl Default for GenericStructPrinter<i32> {
    fn default() -> Self {
        Self {
            phantom_data: PhantomData::default()
        }
    }
}

