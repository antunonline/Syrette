#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

mod bootstrap;
mod interfaces;
mod ninja;

use std::error::Error;

use syrette::di_container::blocking::prelude::*;

use crate::bootstrap::bootstrap;
use crate::interfaces::ninja::INinja;

fn main() -> Result<(), Box<dyn Error>>
{
    println!("Hello, world!");

    let di_container = bootstrap()?;

    let ninja = di_container.get::<dyn INinja>()?.transient()?;

    ninja.throw_shuriken();

    Ok(())
}
