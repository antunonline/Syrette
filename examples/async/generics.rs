use std::fmt::Display;
use std::marker::PhantomData;
use std::sync::Arc;
use syrette::AsyncDIContainer;
use syrette::dependency_history::DependencyHistory;
use syrette::errors::injectable::InjectableError;
use syrette::future::BoxFuture;
use syrette::interfaces::async_injectable::AsyncInjectable;
use syrette::ptr::{ThreadsafeSingletonPtr, TransientPtr};
use syrette_macros::declare_interface;

pub struct SomethingElse {}
#[syrette_macros::injectable(async=true)]
impl SomethingElse {
    fn new() -> Self {
        Self {}
    }
}
pub struct PrinterConfig<C:Send + Sync + Display> {
    __data : PhantomData<C>,
    something_else: Box<SomethingElse>
}

#[syrette::injectable(no_declare_concrete_interface=true, async=true)]
impl<C:Send + Sync + Display + 'static>  PrinterConfig<C> {
    pub fn new(something_else: TransientPtr<SomethingElse>) -> Self {
        Self {
            __data: PhantomData::default(),
            something_else
        }
    }
}

pub trait IPrinter<C: Send + Sync + Display>: Send + Sync {

    fn print(&self, c: &C);
}


pub struct Printer<C: Send + Display + Sync> {
    config: PrinterConfig<C>
}

#[syrette::injectable(no_declare_concrete_interface=true, async=true)]
impl<C: Send + Display + Sync + 'static>  Printer<C> {
    pub fn new(config: TransientPtr<PrinterConfig<C>>) -> Self {
        Self {
            config: *config
        }
    }
    pub fn print(&self, c: &C) {
        println!("{}", c);
    }
}

impl<C: Send + Display + Sync + 'static> IPrinter<C> for  Printer<C> {
    fn print(&self, c: &C) {
       self.print(c);
    }
}



declare_interface!(Printer<String> -> Printer<String>, threadsafe_sharable=true);
declare_interface!(Printer<String> -> IPrinter<String>, threadsafe_sharable=true);
declare_interface!(PrinterConfig<String> -> PrinterConfig<String>, threadsafe_sharable=true);


pub struct SomeExample<C: Send + Sync> {
    data: PhantomData<C>
}


impl <C: Send + Sync+ 'static> SomeExample<C> {
    pub fn new() -> Self {
        Self {
            data: PhantomData::default()
        }
    }
}
