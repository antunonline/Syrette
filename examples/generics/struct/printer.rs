use std::fmt::Display;
use std::marker::PhantomData;

pub struct StructPrinter<Msg: Send + Display> {
    phantom_data: PhantomData<Msg>,
}


impl<Msg: Send + Display> StructPrinter<Msg> {
    pub fn print(&self, msg: &Msg) {
        println!("{}", msg);
    }
}


impl Default for StructPrinter<String> {
    fn default() -> Self {
        Self {
            phantom_data: PhantomData::default()
        }
    }
}

impl Default for StructPrinter<i32> {
    fn default() -> Self {
        Self {
            phantom_data: PhantomData::default()
        }
    }
}

